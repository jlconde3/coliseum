# COLISEUM NET

This Rust project implements a distributed system where nodes communicate with each other over TCP connections and handle requests from clients. It provides functionality for creating, retrieving, and distributing items among nodes in the network.

## Proceso de creación de un item

1) Un cliente solicita crear un item contra un nodo por medio de una conexión TCP. Esta solicitud contiene el cuerpo del item/datos.
2) El nodo asignará un identificador único al item.
3) El nodo asignará una fecha de creación al item.
4) El contenido o datos del item se envián desde el cliente y son procesados por el nodo para crear el item.
5) El nodo devuelve el item al cliente.
6) El nodo envía a cada uno de los nodos de la red el nuevo item. Si a la hora del envío no es posible conectar
con el nodo esto implica que el nodo de destino no está disponible y por lo tanto se elimina de la lista de nodos.
7) El resto de nodos almacenan el nuevo item.

## Proceso de recuperación de un item en la red

1) Un cliente solicita un item contra un nodo de la red por medio de una conexión TCP. Esta solicitud contiene el id del item a recueprar.
2) El nodo procesa la solicitud y comprueba si el item está almecenado en la lista de items del nodo.
3) Si el item es o no encontrado el nodo realizará una llamada a cada uno de los nodos preguntando por el item.
   1) Si estos le devuelve al primer nodo el item este lo almacenará de manera momentánea.
   2) Preguntará como mínimo al 51% de los nodos de manera que se tiene una muestra lo suficientemete grande para
   encontrar el item y evitar problemas de control en la red.
   3) Solo se considera que una conexión es correcta y suma al 51% cuando la conexión es posible y el valor devuelto por
   el segundo nodo es correcto, en caso negativo se elimina al nodo de la lista de nodos.
4) Una vez recibido el número suficiente de items escogerá el item más frecuente para el id solicitado.

