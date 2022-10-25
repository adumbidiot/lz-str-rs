const CHUNK_SIZE = 10_000;

function convertU16SliceToString(slice) {
    let ret = '';
    let num_chunks = Math.ceil(slice.length / CHUNK_SIZE);
    for(let i = 0; i < slice.length; i += CHUNK_SIZE) {
        let end_index = Math.min(i + CHUNK_SIZE, i + slice.length);
        ret += String.fromCharCode(...slice.subarray(i, end_index));
    }
    
    return ret;
}