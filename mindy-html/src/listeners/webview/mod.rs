use crate::load_json_data;
use ::web_sys::window;
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::JsCast;
use web_sys::MessageEvent;


pub fn activate_message_listener() {
    let window = window().expect("Cannot get window");
    let closure = Closure::<dyn FnMut(_)>::new(move |event: MessageEvent| {

        match event.origin().as_str() {
            origin if origin.contains("http://") => load_json_data(DATA_JSON),
            _ => {}
        }

        // Lecture des données du message
        match event.data().as_string() {
            Some(data) => load_json_data(data.as_str()),
            None => return
        };

    });

    window
        .add_event_listener_with_callback("message", closure.as_ref().unchecked_ref())
        .expect("Failed to add event listener");

    closure.forget();
}

const DATA_JSON: &str = r#"
    {
        "metadata": {
            "diagram_type": "Standard"
        },
        "data": {
            "text": "Node 0",
            "children": [
                {
                    "text": "Node 1",
                    "position_direction": "Left",
                    "children": [
                        {
                            "text": "Node 1.1",
                            "children": []
                        }
                    ]
                },
                {
                    "text": "Node 2",
                    "children": [
                        {
                            "text": "Node 123NodeNode 123NodeNode 123NodeNode 123NodeNode 123NodeNode 123NodeNode 123NodeNode 123NodeNode 123Node 123Node 123Node 123Node 123Node 123Node 123Node 123Node 123",
                            "children": []
                        },
                        {
                            "text": "Node 2.2",
                            "children": []
                        },
                        {
                            "text": "Node 2.3",
                            "children": []
                        }
                    ]
                },
                {
                    "text": "Node 3",
                    "children": []
                },
                {
                    "text": "Node 4",
                    "children": []
                },
                {
                    "text": "Node 5",
                    "children": []
                },
                {
                    "text": "Node 6",
                    "children": []
                }
            ]
        }
    }
"#;