package flash.display {
    import flash.utils.ByteArray;
    import flash.display.ShaderData;

    public final class Shader {
        private var _byteCode: ByteArray;
        private var _data: ShaderData;
        private var _precisionHint: String;

        public function Shader(code:ByteArray = null) {
            this._byteCode = code;
        }

        public function set byteCode(val:ByteArray): void {
            this._byteCode = val;
        }

        public function get data(): ShaderData {
            return this._data;
        }

        public function set data(val: ShaderData) : void {
            this._data = val;
        }

        public function get precisionHint(): String {
            return this._precisionHint;
        }

        public function set precisionHint(val: String): void {
            this._precisionHint = val;
        }

    }
}
