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

# **2.3 Small changes, add IP and Port**

![Screenshot server 1](/assets/images/screenshot-server-1.png)
![Screenshot client 1](/assets/images/screenshot-client-1.png)
![Screenshot client 1](/assets/images/screenshot-client-2.png)

- The server now forwards each message with the sender address included, using the client's socket address (IP and port) as the identifier.
- This means we do not need a name registry yet; every client sees who sent the message by the sender's `IP:PORT`.
- The client receives the formatted text from the server and displays it as a message from another client.

**Explanation:** I changed the message format so sender identity is carried by the IP and port. This is useful when clients do not have user names yet, and it allows everyone to know which remote client sent each WebSocket message. 