use crate::core::ScreenCoordinates;

/// The area of a monitor not occupied by global task bars or menu bars is the
/// work area. This is specified in screen coordinates and can be retrieved with
/// [Monitor::work_area](crate::monitor::Monitor::work_area).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct WorkArea
{
	pub pos:  ScreenCoordinates,
	pub size: ScreenCoordinates,
}

impl Default for crate::monitor::WorkArea
{
	fn default() -> crate::monitor::WorkArea
	{
		crate::monitor::WorkArea {
			pos:  ScreenCoordinates::default(),
			size: ScreenCoordinates::default(),
		}
	}
}
