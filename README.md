# Coliseum

## Objetivo

El propósito fundamental de la aplicación Coliseum es desarrollar un sistema de mensajería que sirva como plataforma de aprendizaje para fortalecer los conocimientos en programación concurrente. Este proyecto aborda dos estrategias fundamentales para la programación concurrente:

### Programación Concurrente Basada en Hilos (MultiThreading)

- Esta estrategia implica la creación de múltiples hilos dentro de un proceso principal para manejar diversas tareas simultáneamente.
- La comunicación entre clientes y servidor se establece mediante el protocolo TCP/IP para garantizar la transmisión fiable de datos.
- La seguridad se refuerza mediante la implementación del protocolo SSL/TLS, autenticando la identidad del servidor y cifrando la comunicación para preservar la confidencialidad.

### Programación Concurrente Basada en Bucles de Eventos (AsyncIO)

- En lugar de utilizar hilos, AsyncIO se basa en un único hilo para gestionar de manera eficiente operaciones de entrada/salida sin bloquear el hilo principal.
- La comunicación persistente se logra a través del bucle de eventos de AsyncIO, proporcionando una alternativa más eficiente a la programación basada en hilos.
- El protocolo SSL/TLS y TCP/IP continúan siendo fundamentales para la autenticación del servidor y la seguridad en la transmisión de datos.

## Arquitectura

La arquitectura de Coliseum sigue el modelo Cliente-Servidor para facilitar la comunicación entre los usuarios. En cuanto a los protocolos, la aplicación utiliza principalmente TCP/IP para asegurar la transmisión confiable de datos entre dos puntos en una red, empleando direcciones IP.

Para robustecer la seguridad de la comunicación, se incorpora el protocolo SSL/TLS tanto en el cliente como en el servidor. SSL (Secure Sockets Layer) y su sucesor TLS (Transport Layer Security) son protocolos de seguridad que no solo autentican la identidad del servidor, sino que también establecen un canal de comunicación cifrado. La certificación de la legitimidad del servidor es esencial para prevenir ataques de intermediarios maliciosos. La adopción de SSL/TLS no solo garantiza la autenticidad del servidor, sino que también proporciona un nivel adicional de privacidad y confidencialidad mediante la encriptación de los datos transmitidos, protegiendo la integridad de información sensible como los mensajes de los usuarios durante su tránsito por la red.

### Servidor

Dentro del archivo `server.py`, encontramos la clase que representa al Servidor o Sala donde los clientes se conectan. La conexión se establece mediante la librería `socket`, generando una conexión persistente entre los distintos clientes y el servidor. El servidor tiene la responsabilidad de asignar un identificador único a cada cliente que se conecta y de verificar que el número de usuarios no supere los 3. En caso de una nueva conexión con la sala llena, el servidor envía un mensaje al cliente indicando la capacidad completa y cierra la conexión.

### Cliente

En el lado del cliente (`client.py`), la clase que representa al cliente realiza una primera conexión inicial para verificar la disponibilidad en la sala. En caso de que la sala esté completa, el cliente recibe un mensaje del servidor y cierra la comunicación.

## Diferencias entre las estrategias

Ambas estrategias, MultiThreading y asyncio, logran el mismo resultado, pero difieren significativamente en el funcionamiento interno de la aplicación, afectando el rendimiento y la gestión de recursos del ordenador.

### Modelo Threading

En el servidor, la clase `Server` se ejecuta como un solo proceso que, al conectarse nuevos clientes, crea un nuevo hilo para cada uno dentro del mismo proceso. Estos hilos o clientes acceden a recursos compartidos, gestionando el acceso mediante elementos de bloqueo/sincronización como objetos `threading.Lock` y eventos como `threading.Event` en las clases `Server` y `Client`. Este enfoque permite escalar horizontalmente el número de salas fácilmente. La clase `ClientHandler` hereda de la clase `threading.Thread`, creando un nuevo hilo asociado al objeto `Server` cada vez que se ejecuta mediante `ClientHandler.run`. La persistencia en la comunicación se logra mediante un bucle `ClientHandler.run` que verifica si el cliente envía mensajes, distribuyendo los mensajes entre clientes a través del método `ClientHandler._broadcast`.

En el cliente, la comunicación persistente se logra con dos métodos que funcionan de manera "paralela": `Client._send_message` y `Client._receive_message`. Cada uno tiene un bucle infinito que se cierra si se pierde la conexión con el servidor o si el cliente la cierra.

### Modelo asyncio

En el modelo asyncio, basado en la librería estándar asyncio, se adopta una aproximación distinta a la programación concurrente en comparación con los hilos múltiples, ejecutándose en un único hilo. Esta estrategia optimiza el rendimiento y la gestión de recursos, eliminando la necesidad de operaciones de cambio entre hilos.

En la implementación del servidor, se sigue una estrategia similar al modelo de hilos múltiples, aunque con una filosofía diferente. En asyncio, se utiliza un único bucle de eventos que gestiona la cola de eventos, proporcionándoles acceso a los recursos del servidor. A diferencia del modelo de hilos múltiples, no es necesario crear tareas concurrentes, ya que el sistema se basa en eventos.

En el cliente, la implementación se vuelve más compleja debido a la necesidad de llevar a cabo tareas en paralelo, como la recepción y el envío de mensajes. Un escenario común es que el cliente esté redactando un mensaje mientras recibe otro. Sin embargo, se ha identificado una limitación en la función input que bloquea la visualización de mensajes recibidos hasta que se introduce un nuevo mensaje.


## Conclusiones

### MultiThreading

La estrategia basada en hilos se centra en la creación de múltiples hilos dentro de un proceso principal. Cada hilo representa una ejecución independiente de código que comparte recursos comunes. La filosofía radica en la concurrencia de tareas simultáneas, permitiendo escalar horizontalmente al agregar más hilos. Elementos de sincronización como bloqueos y eventos se utilizan para prevenir conflictos en el acceso a recursos compartidos.

### AsyncIO

AsyncIO, por otro lado, adopta un enfoque basado en un bucle de eventos. Un único hilo maneja operaciones asíncronas, gestionando eventos en una cola. Esto mejora la eficiencia al evitar operaciones costosas de cambio de contexto entre hilos. La programación asíncrona simplifica el diseño y permite una gestión eficiente de la concurrencia sin depender de múltiples hilos.
