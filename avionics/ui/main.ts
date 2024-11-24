#!/usr/bin/env -S deno run -A --unstable-net --watch
const l = Deno.listenDatagram({ transport: "udp", port: 0 });
Deno.serve(async (req) => {
  if (req.headers.has("upgrade")) {
    const { response, socket } = Deno.upgradeWebSocket(req);
    socket.onmessage = (m) => {
      l.send(new Uint8Array(m.data), {
        transport: "udp",
        hostname: "0.0.0.0",
        port: 42069,
      }).catch(console.error);
    };
    return response;
  }
  return new Response(
    await Deno.readFile(new URL("./main.html", import.meta.url)),
  );
});
