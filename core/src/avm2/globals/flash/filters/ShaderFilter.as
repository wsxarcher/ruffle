package flash.filters {
    import flash.display.Shader;
	public final class ShaderFilter extends BitmapFilter {
		public var bottomExtension : int;
        public var leftExtension : int;
        public var rightExtension : int;
        public var topExtension : int;
        public var shader : Shader;

		public function ShaderFilter(shader:Shader = null) 
		{
			this.bottomExtension = 0;
            this.topExtension = 0;
            this.rightExtension = 0;
            this.leftExtension = 0;
            this.shader = shader;
		}

		override public function clone(): BitmapFilter {
			return new ShaderFilter(this.shader);
		}
	}
}
