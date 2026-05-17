# **2.1 Modifying the websocket port** 

### **Is it also using the same websocket protocol?**

##### **Port and Protocol**

- The server listens on `127.0.0.1:8080`.
- The client connects to `ws://127.0.0.1:8080`.
- Both sides use the same WebSocket protocol.

### **Where is it defined?**
##### **Server**

- Defined in `src/bin/server.rs`.
- The listen address is configured with `LISTEN_ADDR`.
- It accepts incoming TCP connections and upgrades them to WebSocket.

##### **Client**

- Defined in `src/bin/client.rs`.
- The WebSocket URI is configured with `WS_URI`.
- It connects to the server on the same port and sends/receives WebSocket messages.
