#![deny(clippy::all)]

#[cfg(all(
    not(all(target_arch = "wasm32", target_os = "unknown")),
    not(all(
        feature = "test-support",
        any(target_os = "macos", target_os = "windows")
    ))
))]
use napi::bindgen_prelude::*;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
use napi_derive::napi;

mod accessibility;
#[cfg(target_os = "macos")]
mod app_menu;
mod automation;
mod color;
mod custom_elements;
mod diff;
mod element_tree;
mod markdown;
mod motion;
mod renderer;
// The data model is public so `examples/bench_serde.rs` measures the real
// types instead of a copy that silently drifts from them.
pub mod retained_tree;
pub mod style;
mod syntax;
mod text;
mod theme;
// Desktop only. HTTP goes through reqwest_client, not crates.io reqwest.
#[cfg(not(target_family = "wasm"))]
mod updater;

#[cfg(all(
    feature = "test-support",
    any(target_os = "macos", target_os = "windows")
))]
mod test_renderer;

pub use custom_elements::{
    custom_surface, register_custom_element, wire_standard_events, CustomElement,
    CustomElementFactory, CustomRenderContext,
};
pub use element_tree::*;
/// The GPUI this renderer is built on. A custom element must use this copy:
/// a second GPUI in the same addon cannot draw into GPUIX's window.
pub use gpui;
pub use renderer::*;
pub use style::*;

#[cfg(any(
    test,
    not(all(
        feature = "test-support",
        any(target_os = "macos", target_os = "windows")
    ))
))]
const TEST_GPUIX_RENDERER_UNAVAILABLE: &str = concat!(
    "TestGpuixRenderer is macOS and Windows only. ",
    "Linux builds have no test-support because wgpu cannot read a rendered image back yet. ",
    "GpuixRenderer still works on Linux."
);

/// True only when this binary compiled the real GPU test renderer.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[napi]
pub fn has_test_gpuix_renderer() -> bool {
    cfg!(all(
        feature = "test-support",
        any(target_os = "macos", target_os = "windows")
    ))
}

#[cfg(all(
    not(all(target_arch = "wasm32", target_os = "unknown")),
    not(all(
        feature = "test-support",
        any(target_os = "macos", target_os = "windows")
    ))
))]
#[napi]
pub struct TestGpuixRenderer;

#[cfg(all(
    not(all(target_arch = "wasm32", target_os = "unknown")),
    not(all(
        feature = "test-support",
        any(target_os = "macos", target_os = "windows")
    ))
))]
#[napi]
impl TestGpuixRenderer {
    #[napi(constructor)]
    pub fn new(_width: Option<f64>, _height: Option<f64>) -> Result<Self> {
        Err(Error::from_reason(TEST_GPUIX_RENDERER_UNAVAILABLE))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unavailable_message_explains_linux() {
        assert!(TEST_GPUIX_RENDERER_UNAVAILABLE.contains("macOS and Windows only"));
        assert!(TEST_GPUIX_RENDERER_UNAVAILABLE.contains(
            "Linux builds have no test-support because wgpu cannot read a rendered image back yet"
        ));
        assert!(TEST_GPUIX_RENDERER_UNAVAILABLE.contains("GpuixRenderer still works on Linux"));
    }

    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    #[test]
    fn has_test_gpuix_renderer_matches_real_impl() {
        assert_eq!(
            has_test_gpuix_renderer(),
            cfg!(all(
                feature = "test-support",
                any(target_os = "macos", target_os = "windows")
            ))
        );
    }

    #[cfg(all(
        not(all(target_arch = "wasm32", target_os = "unknown")),
        not(all(
            feature = "test-support",
            any(target_os = "macos", target_os = "windows")
        ))
    ))]
    #[test]
    fn stub_constructor_explains_why() {
        match TestGpuixRenderer::new(None, None) {
            Ok(_) => panic!("stub constructor must fail"),
            Err(err) => assert!(
                err.to_string().contains(TEST_GPUIX_RENDERER_UNAVAILABLE),
                "{err}"
            ),
        }
    }
}

/// The imports a downstream addon makes. Every path here must resolve through
/// the crate-root re-exports; `crate::renderer` and `crate::custom_elements` are
/// deliberately not imported, so moving one of these names back to `pub(crate)`
/// breaks this build.
#[cfg(all(test, not(all(target_arch = "wasm32", target_os = "unknown"))))]
mod addon_surface_tests {
    use std::sync::{Arc, Mutex};

    use crate::{
        custom_surface, emit_event_full, register_custom_element, wire_standard_events,
        CustomRenderContext, EventCallback, EventPayload,
    };

    #[test]
    fn an_addon_names_the_surface_from_the_crate_root_and_emits_an_event() {
        // Each item resolves through its crate-root re-export; moving one back
        // to `pub(crate)` stops this test from compiling.
        let _register = register_custom_element;
        let _surface: fn(
            gpui::Stateful<gpui::Div>,
            &CustomRenderContext,
        ) -> gpui::Stateful<gpui::Div> = custom_surface;
        // `Stateful<Div>` is the root the built-in adapters wire here.
        let _wire = wire_standard_events::<gpui::Stateful<gpui::Div>>;

        let seen = Arc::new(Mutex::new(Vec::new()));
        let callback: EventCallback = {
            let seen = seen.clone();
            Arc::new(move |payload: EventPayload| seen.lock().unwrap().push(payload))
        };

        emit_event_full(&Some(callback), 7, "click", |payload| {
            payload.button = Some(0);
        });

        let seen = seen.lock().unwrap();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].element_id, 7.0);
        assert_eq!(seen[0].event_type, "click");
        assert_eq!(seen[0].button, Some(0));
    }
}
