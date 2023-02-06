extern crate alloc;

mod macros;

pub mod errhandlingapi;
pub mod hidpi;
pub mod libloaderapi;
pub mod win_types;
pub mod winbase;
pub mod winuser;

/* TODO:

https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidsdi/nf-hidsdi-hidd_getproductstring

https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidsdi/nf-hidsdi-hidd_getpreparseddata

*/
