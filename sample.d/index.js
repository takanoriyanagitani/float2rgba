(() => {
  const root = document.getElementById("root");

  return Promise.resolve("float2rgba.wasm")
    .then(fetch)
    .then(WebAssembly.instantiateStreaming)
    .then((wasm) => {
      const {
        module,
        instance,
      } = wasm || {};

      const {
        memory,

        float2rgba32u_ext_turbo,
        ext_colorgrad_turbo_init,
      } = instance?.exports || {};

      if (0 != ext_colorgrad_turbo_init()) {
        console.error("unable to initialize");
        return;
      }

      const cnvs = document.getElementById("cnvs");
      const ii = cnvs.getContext("2d");

      const left = 0;
      const top = 0;
      const width = 256;
      const height = 256;

      const img = ii.getImageData(left, top, width, height);
      const dat = img.data;

      const rcp = 1.0 / 256.0;

      const started = Date.now();
      for (let y = 0; y < height; y++) {
        for (let x = 0; x < width; x++) {
          const ix = y * width + x;
          const i = ix << 2;
          const f = rcp * x;

          const u = float2rgba32u_ext_turbo(f);

          dat[i + 0] = (u >> 24) & 0xff;
          dat[i + 1] = (u >> 16) & 0xff;
          dat[i + 2] = (u >> 8) & 0xff;
          dat[i + 3] = u & 0xff;
        }
      }
      const elapsed = Date.now() - started;

      ii.putImageData(img, 0, 0);

      console.info({ elapsed });
    })
    .then((_) => {
      root.textContent = "Loaded.";
    })
    .catch((e) => {
      root.textContent = "ERROR!";
      console.warn(e);
    });
})();
