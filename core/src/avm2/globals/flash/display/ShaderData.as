package flash.display {
    import flash.utils.ByteArray;

    public final class ShaderData {
        private var _byteCode: ByteArray;
        
        public function ShaderData(byteCode:ByteArray) {
            this._byteCode = byteCode;
        }
    }
}
