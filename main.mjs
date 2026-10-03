import { readFile } from "node:fs/promises"; // to read wasm
import { open } from "node:fs/promises"; // to read from stdin(fd 0)

const io_wasmName = () => Promise.resolve("./rs_zstd2raw4wasm.wasm");

const io_main = () => {
  const obj = {};
  obj.memory = null;
  obj.stdin = null;

  return Promise.resolve()
  .then(_ => open("/dev/stdin"))
  .then(stdin => {
    obj.stdin = stdin;
  }).then(_ => {

    const imports = Object.freeze({
      env: {
        host_to_wasm: new WebAssembly.Suspending(
          async (ptr2buf, buflen) => {
            if(!obj.memory) return -1;

            if(buflen <= 0) return 0;

            const wview = new Uint8Array(
              obj.memory.buffer,
              ptr2buf,
              buflen,
            );

            const rslt = await obj.stdin.read(
              wview,
              Object.freeze({
                offset: 0,
                position: null,
              }),
            );

            const { bytesRead } = rslt;

            return bytesRead;
          },
        ),
      },
    });

    const wbytes2instance = (wbytes) => WebAssembly.instantiate(
      wbytes,
      imports,
    );

    const iwname = io_wasmName();
    const iwcontent = iwname.then(readFile);
    const iwasm = iwcontent.then(wbytes2instance);
    const iins = iwasm.then(w => w.instance);
    const iexp = iins.then(i => i.exports);

    return iexp;
  })
  .then(exp => {
    const {
      memory,
      decode_zstd,
      out_pointer,
    } = exp;

    obj.memory = memory;

    const pdec = WebAssembly.promising(decode_zstd);

    return pdec().then(sz => {
      const ptr = out_pointer();
      return {
        sz,
        ptr,
        memory,
      }
    })
  })
  .then(rslt => {
    const {
      sz,
      ptr,
      memory,
    } = rslt;
    const decoded = new Uint8Array(memory.buffer, ptr, sz);
    process.stdout.write(decoded);
  })
  .finally(_ => obj.stdin.close())
  ;
};

io_main()
.catch(console.error)
