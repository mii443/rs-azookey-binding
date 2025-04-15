use std::os::raw::c_void;

use libc::c_int;

/*
@_silgen_name("CreateKanaKanjiConverter")
@MainActor public func create_kana_kanji_converter() -> UnsafeMutablePointer<KanaKanjiConverter> {
    let converter = KanaKanjiConverter()
    return Unmanaged.passRetained(converter).toOpaque().assumingMemoryBound(
        to: KanaKanjiConverter.self)
}
@_silgen_name("DestroyKanaKanjiConverter")
@MainActor public func destroy_kana_kanji_converter(
    _ converter: UnsafeMutablePointer<KanaKanjiConverter>
) {
    Unmanaged<KanaKanjiConverter>.fromOpaque(converter).release()
}
@_silgen_name("CreateComposingText")
@MainActor public func create_composing_text() -> UnsafeMutablePointer<ComposingTextWrapper> {
    let c = ComposingTextWrapper(value: ComposingText())
    return Unmanaged.passRetained(c).toOpaque().assumingMemoryBound(to: ComposingTextWrapper.self)
}
@_silgen_name("DestroyComposingText")
@MainActor public func destroy_composing_text(
    _ composingText: UnsafeMutablePointer<ComposingTextWrapper>
) {
    Unmanaged<ComposingTextWrapper>.fromOpaque(composingText).release()
}
@_silgen_name("KanaKanjiConverter_RequestCandidates")
@MainActor public func kana_kanji_converter_request_candidates(
    _ converter: UnsafeMutablePointer<KanaKanjiConverter>,
    _ composingText: UnsafeMutablePointer<ComposingTextWrapper>,
    _ context: UnsafePointer<CChar>
) -> UnsafeMutablePointer<UnsafeMutablePointer<FFICandidate>> {
    let c = Unmanaged<KanaKanjiConverter>.fromOpaque(converter).takeUnretainedValue()
    let ct = Unmanaged<ComposingTextWrapper>.fromOpaque(composingText).takeUnretainedValue()

    let options = ConvertRequestOptions.withDefaultDictionary(
        requireJapanesePrediction: true,
        requireEnglishPrediction: false,
        keyboardLanguage: .ja_JP,
        learningType: .nothing,
        memoryDirectoryURL: URL(filePath: "./"),
        sharedContainerURL: URL(filePath: "./"),
        zenzaiMode: .on(
            weight: URL(filePath: "./ggml-model-Q5_K_M.gguf"), inferenceLimit: 1,
            requestRichCandidates: true, personalizationMode: nil,
            versionDependentMode: .v3(.init(profile: "", leftSideContext: String(cString: context)))
        ),
        preloadDictionary: true,
        metadata: .init(versionString: "rs-azookey-binding")
    )

    let candidates: ConversionResult = c.requestCandidates(ct.value, options: options)

    var result: [FFICandidate] = []

    for i in 0..<candidates.mainResults.count {
        let candidate = candidates.mainResults[i];
        let text = candidate.text
        let correspondingCount = candidate.correspondingCount

        result.append(
            FFICandidate(
                text: UnsafeMutablePointer(mutating: (text as NSString).utf8String!),
                correspondingCount: Int32(correspondingCount),
            )
        )
    }

    let pointer = UnsafeMutablePointer<UnsafeMutablePointer<FFICandidate>>.allocate(capacity: result.count)
    for i in 0..<result.count {
        pointer[i] = UnsafeMutablePointer<FFICandidate>.allocate(capacity: 1)
        pointer[i].pointee = result[i]
    }
    return pointer
}
@_silgen_name("KanaKanjiConverter_StopComposition")
@MainActor public func kana_kanji_converter_stop_composition(
    _ converter: UnsafeMutablePointer<KanaKanjiConverter>,
    _ composingText: UnsafeMutablePointer<ComposingTextWrapper>
) {
    let c = Unmanaged<KanaKanjiConverter>.fromOpaque(converter).takeUnretainedValue()
    c.stopComposition()
}
@_silgen_name("ComposingText_InsertAtCursorPosition")
@MainActor public func composing_text_insert_at_cursor_position(
    _ composingText: UnsafeMutablePointer<ComposingTextWrapper>,
    _ text: UnsafePointer<CChar>,
) {
    let ct = Unmanaged<ComposingTextWrapper>.fromOpaque(composingText).takeUnretainedValue()
    let str = String(cString: text)
    ct.value.insertAtCursorPosition(str, inputStyle: .roman2kana)
}
@_silgen_name("ComposingText_DeleteForwardFromCursorPosition")
@MainActor public func composing_text_delete_forward_from_cursor_position(
    _ composingText: UnsafeMutablePointer<ComposingTextWrapper>,
    _ count: Int32
) {
    let ct = Unmanaged<ComposingTextWrapper>.fromOpaque(composingText).takeUnretainedValue()
    ct.value.deleteForwardFromCursorPosition(count: Int(count))
}
@_silgen_name("ComposingText_DeleteBackwardFromCursorPosition")
@MainActor public func composing_text_delete_backward_from_cursor_position(
    _ composingText: UnsafeMutablePointer<ComposingTextWrapper>,
    _ count: Int32
) {
    let ct = Unmanaged<ComposingTextWrapper>.fromOpaque(composingText).takeUnretainedValue()
    ct.value.deleteBackwardFromCursorPosition(count: Int(count))
}

*/
unsafe extern "C" {
    pub fn CreateKanaKanjiConverter() -> *mut c_void;
    pub fn DestroyKanaKanjiConverter(converter: *mut c_void);
    pub fn CreateComposingText() -> *mut c_void;
    pub fn DestroyComposingText(composingText: *mut c_void);
    pub fn KanaKanjiConverter_RequestCandidates(
        converter: *mut c_void,
        composingText: *mut c_void,
        lengthPtr: *mut c_int,
        context: *const libc::c_char,
    ) -> *mut *mut FFICandidate;
    pub fn KanaKanjiConverter_StopComposition(converter: *mut c_void);
    pub fn ComposingText_InsertAtCursorPosition(
        composingText: *mut c_void,
        text: *const libc::c_char,
    );
    pub fn ComposingText_DeleteForwardFromCursorPosition(
        composingText: *mut c_void,
        count: libc::c_int,
    );
    pub fn ComposingText_DeleteBackwardFromCursorPosition(
        composingText: *mut c_void,
        count: libc::c_int,
    );
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct FFICandidate {
    text: *mut libc::c_char,
    corresponding_count: libc::c_int,
}

#[derive(Debug, Clone)]
pub struct Candidate {
    pub text: String,
    pub corresponding_count: i32,
}

pub struct KanaKanjiConverter {
    pub converter: *mut c_void,
}

pub struct ComposingText {
    pub composing_text: *mut c_void,
}

impl KanaKanjiConverter {
    pub fn new() -> Self {
        unsafe {
            let converter = CreateKanaKanjiConverter();
            if converter.is_null() {
                panic!("Failed to create KanaKanjiConverter");
            }
            Self { converter }
        }
    }

    pub fn request_candidates(
        &self,
        composing_text: &ComposingText,
        context: &str,
    ) -> Vec<Candidate> {
        unsafe {
            let c_str = std::ffi::CString::new(context).expect("CString::new failed");
            let mut length: c_int = 0;
            let candidates_ptr = KanaKanjiConverter_RequestCandidates(
                self.converter,
                composing_text.composing_text,
                &mut length,
                c_str.as_ptr(),
            );
            if candidates_ptr.is_null() {
                panic!("Failed to get candidates");
            }

            let mut candidates = Vec::new();
            for i in 0..length {
                let candidate = *(*candidates_ptr.offset(i as isize));
                let text = std::ffi::CStr::from_ptr(candidate.text)
                    .to_string_lossy()
                    .into_owned();
                let corresponding_count = candidate.corresponding_count;
                candidates.push(Candidate {
                    text,
                    corresponding_count,
                });
            }
            candidates
        }
    }

    pub fn stop_composition(&self) {
        unsafe {
            KanaKanjiConverter_StopComposition(self.converter);
        }
    }
}

impl Drop for KanaKanjiConverter {
    fn drop(&mut self) {
        unsafe {
            DestroyKanaKanjiConverter(self.converter);
        }
    }
}

impl ComposingText {
    pub fn new() -> Self {
        unsafe {
            let composing_text = CreateComposingText();
            if composing_text.is_null() {
                panic!("Failed to create ComposingText");
            }
            Self { composing_text }
        }
    }

    pub fn insert_at_cursor_position(&self, text: &str) {
        unsafe {
            let c_str = std::ffi::CString::new(text).expect("CString::new failed");
            ComposingText_InsertAtCursorPosition(self.composing_text, c_str.as_ptr());
        }
    }

    pub fn delete_forward_from_cursor_position(&self, count: i32) {
        unsafe {
            ComposingText_DeleteForwardFromCursorPosition(self.composing_text, count);
        }
    }

    pub fn delete_backward_from_cursor_position(&self, count: i32) {
        unsafe {
            ComposingText_DeleteBackwardFromCursorPosition(self.composing_text, count);
        }
    }
}

impl Drop for ComposingText {
    fn drop(&mut self) {
        unsafe {
            DestroyComposingText(self.composing_text);
        }
    }
}
