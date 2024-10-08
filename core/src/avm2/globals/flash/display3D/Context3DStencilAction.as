// The initial version of this file was autogenerated from the official AS3 reference at
// https://help.adobe.com/en_US/FlashPlatform/reference/actionscript/3/flash/display3D/Context3DStencilAction.html
// by https://github.com/golfinq/ActionScript_Event_Builder
// It won't be regenerated in the future, so feel free to edit and/or fix

package flash.display3D
{

    [API("674")]
    public final class Context3DStencilAction
    {
        // Decrement the stencil buffer value, clamping at 0, the minimum value.
        public static const DECREMENT_SATURATE:String = "decrementSaturate";

        // Decrement the stencil buffer value.
        public static const DECREMENT_WRAP:String = "decrementWrap";

        // Increment the stencil buffer value, clamping at 255, the maximum value.
        public static const INCREMENT_SATURATE:String = "incrementSaturate";

        // Increment the stencil buffer value.
        public static const INCREMENT_WRAP:String = "incrementWrap";

        // Invert the stencil buffer value, bitwise.
        public static const INVERT:String = "invert";

        // Keep the current stencil buffer value.
        public static const KEEP:String = "keep";

        // Replace the stencil buffer value with the reference value.
        public static const SET:String = "set";

        // Set the stencil buffer value to 0.
        public static const ZERO:String = "zero";

    }
}
