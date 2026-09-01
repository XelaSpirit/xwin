#[macro_export]
macro_rules! glfw_enum {
	($type:ty, $rep:ty) => {
		impl $type
		{
			pub(super) fn from_glfw(value: u32) -> $type
			{
				// This does not 100& guarantee a valid value, as some enums don't cover all
				// values within their type, and some (JoystickHatState) even have gaps in their
				// coverage. This check only covers the most obviously wrong values. Production
				// code won't have this assertion, anyway.
				debug_assert!(
					value <= <$rep>::MAX as u32,
					"Attempted to convert invalid glfw enum value"
				);
				unsafe { std::mem::transmute(value as $rep) }
			}
		}
	};
}
