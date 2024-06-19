(() => {
  const root = document.getElementById("root");

  return Promise.resolve("float2rgba.wasm")
    .then(fetch)
    .then(WebAssembly.compileStreaming)
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

        ext_colorgrad_rainbow_init,
        float2rgba32u_ext_rainbow,
      } = instance?.exports || {};

      const rainbow = true;

      const init = rainbow
        ? ext_colorgrad_rainbow_init
        : ext_colorgrad_turbo_init;

      if (0 != init()) {
        return Promise.reject("unable to initialize");
      }

      return WebAssembly.instantiate(
        module,
        {
          env: {
            float2rgba: rainbow
              ? float2rgba32u_ext_rainbow
              : float2rgba32u_ext_turbo,
          },
        },
      );
    })
    .then((instance) => {
      const {
        memory,

        cnvs_imgdat_ext_input_init,
        cnvs_imgdat_ext_output_reset,
        cnvs_imgdat_ext_convert_swap,

        cnvs_imgdat_ext_input_ptr,
        cnvs_imgdat_ext_output_ptr,
      } = instance?.exports || {};

      const width = 256;
      const height = 256;

      const sz = width * height;

      const icap = cnvs_imgdat_ext_input_init(sz, 0.0);
      const ocap = cnvs_imgdat_ext_output_reset(sz);

      console.info({ icap, ocap });

      const iptr = cnvs_imgdat_ext_input_ptr();
      const optr = cnvs_imgdat_ext_output_ptr();

      console.info({ iptr, optr });

      const dview = new DataView(memory.buffer, iptr, sz << 2);

      const rcp = 1.0 / 256.0;

      for (let y = 0; y < height; y++) {
        for (let x = 0; x < width; x++) {
          const ix = y * width + x;
          const f = rcp * x;
          dview.setFloat32(ix << 2, f, true); // JS: big endian by default
        }
      }

      // Wasm: little endian
      const conv_cnt = cnvs_imgdat_ext_convert_swap(true);

      console.info({ conv_cnt });
      const oview = new Uint8Array(memory.buffer, optr, sz << 2);

      const cnvs = document.getElementById("cnvs");
      const ii = cnvs.getContext("2d");

      const left = 0;
      const top = 0;

      const img = ii.getImageData(left, top, width, height);
      const dat = img.data;

      dat.set(oview, 0);
      ii.putImageData(img, 0, 0);
    })
    .then((_) => {
      root.textContent = "Loaded.";
    })
    .catch((e) => {
      root.textContent = "ERROR!";
      console.warn(e);
    });
})();
