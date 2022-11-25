package flash.filters {
	public final class DropShadowFilter extends BitmapFilter {
		public var distance: Number;
		public var angle: Number;
		public var color: uint;
		public var alpha: Number;
		public var blurX: Number;
		public var blurY: Number;
		public var strength: Number;
		public var quality: int;
		public var inner: Boolean;
		public var knockout: Boolean;
		public var hideObject: Boolean;

		public function DropShadowFilter(distance: Number = 4.0,		
                                         angle: Number = 45,
										 color: uint = 0,
										 alpha: Number = 1.0,
										 blurX: Number = 4.0,
										 blurY: Number = 4.0,
										 strength: Number = 1.0,
										 quality: int = 1,
										 inner: Boolean = false,
										 knockout: Boolean = false,
										 hideObject: Boolean = false)
		{
			this.distance = distance;
			this.angle = angle;
			this.color = color;
			this.alpha = alpha;
			this.blurX = blurX;
			this.blurY = blurY;
			this.strength = strength;
			this.quality = quality;
			this.inner = inner;
			this.knockout = knockout;
			this.hideObject = hideObject;
		}

		override public function clone(): BitmapFilter {
			return new DropShadowFilter(this.distance, this.angle, this.color, this.alpha, this.blurX, this.blurY, this.strength, this.quality, this.inner, this.knockout, this.hideObject);
		}
	}
}