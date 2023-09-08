extern crate clipboard;
extern crate device_query;
extern crate trie_rs;

// use clipboard::{
//     ClipboardProvider,
//     windows_clipboard::WindowsClipboardContext,
// };
use device_query::{
    DeviceEvents,
    DeviceState,
    Keycode,
};
use enigo::{
    Enigo,
    Key,
    KeyboardControllable,
};
use trie_rs::TrieBuilder;

use std::borrow::BorrowMut;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct KeyWrapper(Keycode);

impl KeyWrapper {
    fn from_keycode(key: Keycode) -> Self {
        KeyWrapper(key)
    }

    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl PartialOrd for KeyWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for KeyWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0 as u32).cmp(&(other.0 as u32))
    }
}

fn main() {
    let mut trie_builder = TrieBuilder::new();
    load_deny_list_into_trie(&mut trie_builder);
    let trie = trie_builder.build();

    let device_state = DeviceState::new();
    let key_buffer = Arc::new(Mutex::new(Vec::new()));

    let enigo = Arc::new(Mutex::new(Enigo::new()));

    let _guard = device_state.on_key_down(move |key| {
        let mut locked_buffer = key_buffer.lock().unwrap();
        locked_buffer.push(KeyWrapper::from_keycode(*key));

        let predictions = trie.predictive_search(&(*locked_buffer));

        if predictions.is_empty() {
            locked_buffer.clear();
        } else {
            let mut locked_enigo = enigo.lock().unwrap();

            for prediction in &predictions {
                if locked_buffer.as_slice() == prediction.as_slice() {
                    let len = prediction.len();
                    for _ in 0..len {
                        locked_enigo.borrow_mut().key_click(Key::Backspace);
                    }
                    for _ in 0..len {
                        locked_enigo.borrow_mut().key_sequence("*");
                    }

                    println!("sanitized: {:?}", prediction.iter().map(|k| k.to_string()).collect::<Vec<_>>().join(""));
                    locked_buffer.clear();
                }
            }
        }
    });

    loop {
        // TODO: use a separate Trie<char> for parsing clipboard

        // let mut clipboard: WindowsClipboardContext = ClipboardProvider::new().unwrap();
        // let clipboard_content = clipboard.get_contents().unwrap_or_default();
        // let processed_clipboard = process_string(&clipboard_content, &trie);

        // if clipboard_content != processed_clipboard {
        //     clipboard.set_contents(processed_clipboard).unwrap();
        // }

        thread::sleep(Duration::from_secs(5000));
    }
}

fn load_deny_list_into_trie(trie_builder: &mut TrieBuilder<KeyWrapper>) {
    let data = include_str!("deny.txt");

    println!("deny list: ");
    for line in data.lines() {

        let keycodes: Result<Vec<KeyWrapper>, _> = line.chars()
            .map(|c| {
                let s = c.to_string();
                Keycode::from_str(&s)
                    .map(KeyWrapper)
                    .map_err(|e| format!("failed to parse keycode from (try uppercase) {}: {}", s, e))
            })
            .collect();


        match keycodes {
            Ok(kcs) => {
                println!("{:?}", kcs.iter().map(|k| k.to_string()).collect::<Vec<_>>().join(""));
                trie_builder.push(kcs);
            },
            Err(err_msg) => eprintln!("{}", err_msg),
        }
    }
}
