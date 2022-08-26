(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/norma2turing.js":
/*!******************************!*\
  !*** ../pkg/norma2turing.js ***!
  \******************************/
/*! exports provided: main, compile_code, CompilationResult, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./norma2turing_bg.wasm */ \"../pkg/norma2turing_bg.wasm\");\n/* harmony import */ var _norma2turing_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./norma2turing_bg.js */ \"../pkg/norma2turing_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"main\", function() { return _norma2turing_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"main\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"compile_code\", function() { return _norma2turing_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"compile_code\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"CompilationResult\", function() { return _norma2turing_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"CompilationResult\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return _norma2turing_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_throw\"]; });\n\n\n\n_norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_start\"]();\n\n\n//# sourceURL=webpack:///../pkg/norma2turing.js?");

/***/ }),

/***/ "../pkg/norma2turing_bg.js":
/*!*********************************!*\
  !*** ../pkg/norma2turing_bg.js ***!
  \*********************************/
/*! exports provided: main, compile_code, CompilationResult, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"main\", function() { return main; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"compile_code\", function() { return compile_code; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"CompilationResult\", function() { return CompilationResult; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony import */ var _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./norma2turing_bg.wasm */ \"../pkg/norma2turing_bg.wasm\");\n\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachedUint8Memory0 = new Uint8Array();\n\nfunction getUint8Memory0() {\n    if (cachedUint8Memory0.byteLength === 0) {\n        cachedUint8Memory0 = new Uint8Array(_norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachedUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n/**\n*/\nfunction main() {\n    _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"main\"]();\n}\n\nlet cachedInt32Memory0 = new Int32Array();\n\nfunction getInt32Memory0() {\n    if (cachedInt32Memory0.byteLength === 0) {\n        cachedInt32Memory0 = new Int32Array(_norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachedInt32Memory0;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n/**\n* Returns: (compiled_successfully, compilation_info, compiled_file)\n* @param {string} input\n* @returns {CompilationResult}\n*/\nfunction compile_code(input) {\n    const ptr0 = passStringToWasm0(input, _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n    const len0 = WASM_VECTOR_LEN;\n    const ret = _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"compile_code\"](ptr0, len0);\n    return CompilationResult.__wrap(ret);\n}\n\n/**\n*/\nclass CompilationResult {\n\n    static __wrap(ptr) {\n        const obj = Object.create(CompilationResult.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    __destroy_into_raw() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        return ptr;\n    }\n\n    free() {\n        const ptr = this.__destroy_into_raw();\n        _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_compilationresult_free\"](ptr);\n    }\n    /**\n    * @returns {boolean}\n    */\n    get successful() {\n        const ret = _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_get_compilationresult_successful\"](this.ptr);\n        return ret !== 0;\n    }\n    /**\n    * @param {boolean} arg0\n    */\n    set successful(arg0) {\n        _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_set_compilationresult_successful\"](this.ptr, arg0);\n    }\n    /**\n    * @returns {string}\n    */\n    get info() {\n        try {\n            const retptr = _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](-16);\n            _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_get_compilationresult_info\"](retptr, this.ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            return getStringFromWasm0(r0, r1);\n        } finally {\n            _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](16);\n            _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1);\n        }\n    }\n    /**\n    * @param {string} arg0\n    */\n    set info(arg0) {\n        const ptr0 = passStringToWasm0(arg0, _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        const len0 = WASM_VECTOR_LEN;\n        _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_set_compilationresult_info\"](this.ptr, ptr0, len0);\n    }\n    /**\n    * @returns {string}\n    */\n    get filedata() {\n        try {\n            const retptr = _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](-16);\n            _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_get_compilationresult_filedata\"](retptr, this.ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            return getStringFromWasm0(r0, r1);\n        } finally {\n            _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](16);\n            _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1);\n        }\n    }\n    /**\n    * @param {string} arg0\n    */\n    set filedata(arg0) {\n        const ptr0 = passStringToWasm0(arg0, _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        const len0 = WASM_VECTOR_LEN;\n        _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_set_compilationresult_filedata\"](this.ptr, ptr0, len0);\n    }\n    /**\n    * @returns {string}\n    */\n    get filename() {\n        try {\n            const retptr = _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](-16);\n            _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_get_compilationresult_filename\"](retptr, this.ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            return getStringFromWasm0(r0, r1);\n        } finally {\n            _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](16);\n            _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1);\n        }\n    }\n    /**\n    * @param {string} arg0\n    */\n    set filename(arg0) {\n        const ptr0 = passStringToWasm0(arg0, _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        const len0 = WASM_VECTOR_LEN;\n        _norma2turing_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_set_compilationresult_filename\"](this.ptr, ptr0, len0);\n    }\n}\n\nfunction __wbindgen_throw(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/norma2turing_bg.js?");

/***/ }),

/***/ "../pkg/norma2turing_bg.wasm":
/*!***********************************!*\
  !*** ../pkg/norma2turing_bg.wasm ***!
  \***********************************/
/*! exports provided: memory, main, __wbg_compilationresult_free, __wbg_get_compilationresult_successful, __wbg_set_compilationresult_successful, __wbg_get_compilationresult_info, __wbg_set_compilationresult_info, __wbg_get_compilationresult_filedata, __wbg_set_compilationresult_filedata, __wbg_get_compilationresult_filename, __wbg_set_compilationresult_filename, compile_code, __wbindgen_add_to_stack_pointer, __wbindgen_free, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_start */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./norma2turing_bg.js */ \"../pkg/norma2turing_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/norma2turing_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var norma2turing__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! norma2turing */ \"../pkg/norma2turing.js\");\n\r\n\r\nvar codeBlock = document.getElementById(\"norma\");\r\nvar infoTitle = document.getElementById(\"infoTitle\");\r\nvar infoCode = document.getElementById(\"infoCode\");\r\nvar infoText = document.getElementById(\"infoText\");\r\n\r\nvar saveButton = document.getElementById(\"save\");\r\nsaveButton.disabled = true;\r\n\r\nvar fileData = \"// empty\";\r\nvar fileName = \"maqturing.mt\"\r\n\r\ncodeBlock.addEventListener('input', () => {\r\n    let codeText = codeBlock.value;\r\n    var compileResult = norma2turing__WEBPACK_IMPORTED_MODULE_0__[\"compile_code\"](codeText);\r\n\r\n    saveButton.disabled = !compileResult.successful;\r\n    fileData = compileResult.filedata;\r\n    fileName = compileResult.filename;\r\n\r\n    let hash = compileResult.info.indexOf('#');\r\n    setTitle(compileResult.info.substring(0, hash));\r\n    let infoInner = compileResult.info.substring(hash + 1);\r\n\r\n    let sep = infoInner.lastIndexOf('=');\r\n    infoCode.innerHTML = infoInner.substring(0, sep);\r\n    infoText.innerHTML = infoInner.substring(sep + 1);\r\n}, false);\r\n\r\nsaveButton.onclick = () => {\r\n    let download = document.createElement('a');\r\n    download.style.display = 'none';\r\n    download.setAttribute('href', 'data:text/plain;charset=utf-8,' + encodeURIComponent(fileData));\r\n    download.setAttribute('download', fileName);\r\n    download.click();\r\n}\r\n\r\nvar setTitle = (titleString) => {\r\n    infoTitle.innerHTML = titleString;\r\n    switch (titleString) {\r\n        case 'Ok':\r\n            infoTitle.style.color = '#7085b5';\r\n            break;\r\n        case 'Aviso':\r\n            infoTitle.style.color = '#f5ff3d';\r\n            break;\r\n        case 'Erro':\r\n            infoTitle.style.color = '#ff3d3d';\r\n            break;\r\n    }\r\n}\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ })

}]);