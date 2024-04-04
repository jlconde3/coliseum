const net = require('net');

// Función para conectar con el servidor TCP
function conectarConServidor() {
    const client = new net.Socket();

    // Conectar con el servidor en el puerto y la dirección IP especificados
    client.connect(5000, '127.0.0.1', function() {
        console.log('Conectado al servidor');

        // Una vez conectado, puedes enviar datos al servidor

        let request = {
          entity:"CLIENT",
          action:"CREATE",
          data:'Hola servidor, soy el cliente'
        }
        client.write(JSON.stringify(request));
    });

    // Escuchar los datos recibidos del servidor
    client.on('data', function(data) {
        console.log('Datos recibidos del servidor: ' + data);
        
        // Puedes hacer lo que quieras con los datos recibidos aquí
    });

    // Manejar errores de conexión
    client.on('error', function(error) {
        console.error('Error de conexión: ' + error.message);
    });

    // Manejar la conexión cerrada
    client.on('close', function() {
        console.log('Conexión cerrada');
    });
}

// Llamar a la función para conectar con el servidor cuando se pulsa el botón
document.getElementById('create_item').addEventListener('click', conectarConServidor);
