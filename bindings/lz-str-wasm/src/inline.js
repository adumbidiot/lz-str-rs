const CHUNK_SIZE = 10_000;

function convertUint16ArrayToString(array) {
    let ret = '';
    let num_chunks = Math.ceil(array.length / CHUNK_SIZE);
    for(let i = 0; i < array.length; i += CHUNK_SIZE) {
        let end_index = Math.min(i + CHUNK_SIZE, i + array.length);
        ret += String.fromCharCode(...array.subarray(i, end_index));
    }
    
    return ret;
}