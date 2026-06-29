//! DataTable topic documentation pages.

mod advanced;
mod charts_integration;
mod column_definition;
mod columns;
mod data_source;
mod editing;
mod export;
mod rendering;
mod rows;
mod selection;
mod slots;
mod sorting_filtering;
mod state;

#[cfg(feature = "preview")]
pub use advanced::DATATABLEADVANCEDDOC_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use charts_integration::DATATABLECHARTSINTEGRATIONDOC_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use column_definition::DATATABLECOLUMNDEFINITIONDOC_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use columns::DATATABLECOLUMNSDOC_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use data_source::DATATABLEDATASOURCEDOC_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use editing::DATATABLEEDITINGDOC_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use export::DATATABLEEXPORTDOC_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use rendering::DATATABLERENDERINGDOC_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use rows::DATATABLEROWSDOC_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use selection::DATATABLESELECTIONDOC_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use slots::DATATABLESLOTSDOC_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use sorting_filtering::DATATABLESORTINGFILTERINGDOC_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use state::DATATABLESTATEDOC_PREVIEW_REGISTRATION;
