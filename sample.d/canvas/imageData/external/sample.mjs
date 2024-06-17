import { readFile } from "node:fs/promises";

(() => {
  return Promise.resolve("float2rgba.wasm")
    .then(readFile)
    .then((wasm) => WebAssembly.compile(wasm))
    .then((module) =>
      WebAssembly.instantiate(
        module,
        {
          env: {
            float2rgba: (f32) => 0x00ff00ff, // dummy converter
          },
        },
      )
        .then((instance) =>
          Object.freeze({
            module,
            instance,
          })
        )
    )
    .then((mi) => {
      const {
        module,
        instance,
      } = mi || {};

      const {
        ext_colorgrad_turbo_init,
        float2rgba32u_ext_turbo,
      } = instance?.exports || {};

      if (0 != ext_colorgrad_turbo_init()) {
        console.error("unable to initialize");
        return;
      }

      return WebAssembly.instantiate(
        module,
        {
          env: {
            float2rgba: float2rgba32u_ext_turbo,
          },
        },
      );
    })
    .then((instance) => {
      const {
        memory,

        cnvs_imgdat_ext_input_init,
        cnvs_imgdat_ext_output_reset,
        cnvs_imgdat_ext_convert,

        cnvs_imgdat_ext_input_ptr,
        cnvs_imgdat_ext_output_ptr,
      } = instance?.exports || {};

      const width = 256;
      const height = 256;

      const sz = width * height;

      const icap = cnvs_imgdat_ext_input_init(sz, 42.0);
      const ocap = cnvs_imgdat_ext_output_reset(sz);

      console.info({ icap, ocap });

      const iptr = cnvs_imgdat_ext_input_ptr();
      const optr = cnvs_imgdat_ext_output_ptr();

      console.info({ iptr, optr });

      const iview = new Float32Array(memory.buffer, iptr, sz);

      const rcp = 1.0 / 256.0;

      for (let y = 0; y < height; y++) {
        for (let x = 0; x < width; x++) {
          const ix = y * width + x;
          const f = rcp * ix;
          iview[ix] = f;
        }
      }

      const started = Date.now();
      const loop = 64;
      for (let i = 0; i < loop; i++) {
        const conv = cnvs_imgdat_ext_convert();
      }
      const elapsed = Date.now() - started;
      console.info({ elapsed });

      const oview = new Uint32Array(memory.buffer, optr, sz);

      return oview;
    })
    .then(console.info)
    .catch(console.warn);
})();
