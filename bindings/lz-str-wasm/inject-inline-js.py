if __name__ == "__main__":
    js_file_str = None
    with open('pkg/lz_str_wasm.js', 'r') as file:
        js_file_str = file.read()
        
    inline_js = None
    with open('src/inline.js', 'r') as file:
        inline_js = file.read()
      
    js_file_str += '\n' + inline_js
    with open('pkg/lz_str_wasm.js', 'w') as file:
        file.write(js_file_str)