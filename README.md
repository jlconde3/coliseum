# Coliseum

## Objetivo

Desarrollar una aplicación de mensajería con el objetivo de fortalecer los conocimientos en programación concurrente.

Se plantean dos estrategias:

- Programación concurrente basada en hilos (Threads).
- Programación concurrente basada en un bucle de eventos (AsyncIO).

Algunos de los requisitos:

- Limitar el número de usuarios conectados.
- Comunicación persistente entre cliente y servidor.
- Seguridad SSL/TLS.
- Procesos concurrentes para el envío y recepción de mensajes.

## Arquitectura

Arquitectura basada en el modelo Cliente-Servidor.

### Protocolos

El protocolo predominante en la comunicación de la aplicación es el TCP/IP, el cual proporciona una transmisión fiable de datos entre dos puntos en una red a través de direcciones IP.

Para reforzar la seguridad en la comunicación, se implementa el protocolo SSL/TLS tanto en el cliente como en el servidor. SSL (Secure Sockets Layer) y su sucesor TLS (Transport Layer Security) son protocolos de seguridad que permiten autenticar la identidad del servidor y establecer un canal de comunicación cifrado. La certificación de la legitimidad del servidor en la red es crucial para prevenir ataques de intermediarios maliciosos.

La adopción de SSL/TLS en la aplicación no solo asegura la autenticidad del servidor, sino que también proporciona un nivel adicional de privacidad y confidencialidad mediante la encriptación de los datos transmitidos. Esto es esencial para proteger la integridad de la información sensible, como los mensajes de los usuarios, durante su tránsito a través de la red.

### Servidor

En el archivo `server.py`, existe la clase que representa al Servidor/Sala donde los clientes se conectarían. La conexión se hace mediante la librería `socket`, creando una conexión persistente entre los distintos clientes y el servidor.

El servidor es responsable de asignar un identificador a cada cliente que se conecta y de verificar que el número de usuarios no sea superior a 3. En caso de una nueva conexión y si el cupo está lleno, enviaría un mensaje al cliente indicando que la sala está llena y cerraría la conexión.

La clase `ClientHandler` hereda de la clase `Threads.Threading`, por lo cual, cada vez que se crea una nueva instancia y se ejecuta mediante `ClientHandler.run`, se crea un nuevo hilo asociado al objeto `Server`.

La persistencia en la comunicación se consigue mediante un bucle `ClientHandler.run` que verifica si el cliente envía mensajes. Por otro lado, el mensaje enviado por un cliente se distribuye al resto gracias al método `ClientHandler._broadcast`. Este método verifica que el remitente no sea el mismo que el destinatario.

### Cliente

En el lado del cliente (`client.py`), tenemos la clase que representa al cliente, el cual realiza una primera conexión inicial para verificar si es posible acceder a la sala. En caso negativo, recibe un mensaje desde el servidor y cierra la comunicación.

La comunicación persistente se realiza mediante dos métodos que funcionan de manera "paralela": `Client._send_message` y `Client._receive_message`. Cada uno tiene un bucle infinito que se cierra si se pierde la conexión con el servidor o si el cliente la cierra.

## Diferencias entre las estrategias

Aunque ambas estrategias consiguen el mismo resultado, hay diferencias muy importante en el funcionamiento interno de la aplicación que afecta en gran medidad al rendimiento y a la gestión de los distintos recursos del ordenador.

### Modelo Threading

En el servidor, la clase `Server` se ejecuta como un solo proceso del cual, a medida que se conectan los distintos clientes, irá creando un nuevo hilo para cada uno dentro del mismo proceso. Estos hilos/clientes acceden a ciertos recursos compartidos. El control de acceso se realiza mediante elementos de bloqueo/sincronización, como el uso de objetos `Threads.Lock` y eventos como `Threads.Event` en las clases `Server` y `Client`, evitando condiciones de carrera y acceso indebido a estos recursos. Este sistema permite escalar horizontalmente el número de salas fácilmente.

### Modelo asyncio
