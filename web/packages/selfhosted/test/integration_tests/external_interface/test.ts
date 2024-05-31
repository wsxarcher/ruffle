/* eslint-disable @typescript-eslint/no-explicit-any */

import {
    getTraceOutput,
    injectRuffleAndWait,
    openTest,
    playAndMonitor,
} from "../../utils.js";
import { expect, use } from "chai";
import chaiHtml from "chai-html";

use(chaiHtml);

interface RuffleTest {
    set(...args: any[]): string;

    get(): any[];

    called(): boolean;

    log(...args: any[]): void;
}

declare global {
    // eslint-disable-next-line @typescript-eslint/no-namespace
    namespace WebdriverIO {
        interface Element {
            // ExternalInterface makes whatever functions exist "magically" - so let's just declare here for type's sake

            // Logs all arguments to trace
            log: (...args: unknown[]) => void;

            // Calls `ExternalInterface.call(name, ...args)` after a delay (to avoid reentrancy)
            callMethodWithDelay: (name: string, ...args: unknown[]) => void;

            // Calls `ExternalInterface.call(name, ...args)` immediately (causing reentrancy)
            callMethodImmediately: (name: string, ...args: unknown[]) => void;

            // Returns `value`
            returnAValue: <T>(value: T) => T;

            // Should return an exception...
            throwAnException: void;

            // Calls `ExternalInterface.marshallExceptions = value`
            setMarshallExceptions: (value: boolean) => void;
        }
    }

    interface Window {
        RuffleTest: RuffleTest;
    }
}

async function getCalledValue(browser: WebdriverIO.Browser) {
    await browser.waitUntil(
        async () => await browser.execute(() => window.RuffleTest.called()),
    );
    return await browser.execute(() => window.RuffleTest.get());
}

describe("ExternalInterface", () => {
    it("loads the test", async () => {
        await openTest(browser, "integration_tests/external_interface");
        await injectRuffleAndWait(browser);
        const player = await browser.$("<ruffle-object>");
        await playAndMonitor(
            browser,
            player,
            `ExternalInterface.available: true
ExternalInterface.objectID: "flash_name"
`,
        );

        // Set up callbacks
        await browser.execute(() => {
            let values: any[] = [];
            let called = false;
            window.RuffleTest = {
                set: function (...args) {
                    const wasCalled = called;
                    values = args;
                    called = true;
                    return wasCalled ? "already called!" : "success!";
                },
                get: function () {
                    called = false;
                    return values;
                },
                called: function () {
                    return called;
                },
                log: function (...args: any[]) {
                    (
                        document.getElementById(
                            "flash_id",
                        ) as unknown as WebdriverIO.Element
                    ).log(...args);
                },
            };
        });
    });

    it("responds to 'log'", async () => {
        const player = await browser.$("<ruffle-object>");
        await browser.execute(
            (player) =>
                player.log("Hello world!", {
                    key: "value",
                    nested: ["a", "b"],
                }),
            player,
        );

        const actualOutput = await getTraceOutput(browser, player);
        expect(actualOutput).to.eql(
            `log called with 2 arguments
  [
    "Hello world!"
    Object {
        key = "value"
        nested = [
          "a"
          "b"
        ]
      }
  ]
`,
        );
    });

    it("returns a value", async () => {
        const player = await browser.$("<ruffle-object>");
        const returned = await browser.execute(
            (player) => player.returnAValue(123.4),
            player,
        );

        expect(returned).to.eql(123.4);

        const actualOutput = await getTraceOutput(browser, player);
        expect(actualOutput).to.eql(
            `returnAValue called with 123.4
  [
    123.4
  ]
`,
        );
    });

    it("calls a method with delay", async () => {
        const player = await browser.$("<ruffle-object>");
        await browser.execute(
            (player) =>
                player.callMethodWithDelay("window.RuffleTest.set", true),
            player,
        );
        const actualValue = await getCalledValue(browser);
        expect(actualValue).to.eql([true]);

        const actualOutput = await getTraceOutput(browser, player);
        expect(actualOutput).to.eql(
            `callMethodWithDelay called with 2 arguments
  [
    "window.RuffleTest.set"
    true
  ]
  call(window.RuffleTest.set, ...) = "success!"
`,
        );
    });

    // [NA] Broken on Ruffle at time of writing
    it.skip("calls a reentrant JS method", async () => {
        // JS -> Flash -> JS within one call
        const player = await browser.$("<ruffle-object>");
        const actualValue = await browser.execute((player) => {
            player.callMethodImmediately("window.RuffleTest.set", {
                nested: { object: { complex: true } },
            });
            return window.RuffleTest.get();
        }, player);

        expect(actualValue).to.eql({
            nested: { object: { complex: true } },
        });

        const actualOutput = await getTraceOutput(browser, player);
        expect(actualOutput).to.eql(
            `callMethodImmediately called with 2 arguments
  [
    "window.RuffleTest.set"
    Object {
        nested = Object {
          object = Object {
            complex = true
          }
        }
      }
  ]
  call(window.RuffleTest.set, ...) = "success!"
`,
        );
    });

    it("calls a reentrant Flash method", async () => {
        // Flash -> JS -> Flash within one call
        const player = await browser.$("<ruffle-object>");
        await browser.execute((player) => {
            player.callMethodWithDelay("window.RuffleTest.log", "Reentrant!");
        }, player);

        // [NA] Because of the delay, if we fetch immediately we *may* just get part of the log.
        await browser.pause(200);

        const actualOutput = await getTraceOutput(browser, player);
        expect(actualOutput).to.eql(
            `callMethodWithDelay called with 2 arguments
  [
    "window.RuffleTest.log"
    "Reentrant!"
  ]
log called with 1 argument
  [
    "Reentrant!"
  ]
  call(window.RuffleTest.log, ...) = undefined
`,
        );
    });

    it("supports a JS function as name", async () => {
        const player = await browser.$("<ruffle-object>");
        await browser.execute((player) => {
            player.callMethodWithDelay(
                "function(name){window.RuffleTest.set(name)}",
                "test",
            );
        }, player);

        // [NA] Because of the delay, if we fetch immediately we *may* just get part of the log.
        await browser.pause(200);

        const actualValue = await getCalledValue(browser);
        expect(actualValue).to.eql(["test"]);

        const actualOutput = await getTraceOutput(browser, player);
        expect(actualOutput).to.eql(
            `callMethodWithDelay called with 2 arguments
  [
    "function(name){window.RuffleTest.set(name)}"
    "test"
  ]
  call(function(name){window.RuffleTest.set(name)}, ...) = undefined
`,
        );
    });

    it("supports calling a method that doesn't exist", async () => {
        const player = await browser.$("<ruffle-object>");
        await browser.execute((player) => {
            player.callMethodWithDelay("does.not.exist");
        }, player);

        // [NA] Because of the delay, if we fetch immediately we *may* just get part of the log.
        await browser.pause(200);

        const actualOutput = await getTraceOutput(browser, player);
        expect(actualOutput).to.eql(
            `callMethodWithDelay called with 1 argument
  [
    "does.not.exist"
  ]
  call(does.not.exist, ...) = undefined
`,
        );
    });
});
