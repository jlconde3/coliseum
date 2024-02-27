# Coliseum

Aplicación web de mensajería instantanea anónima.

## Requisitos técnicos

- Pantalla de inicio
- Presiona ENTER para conectar contra el servicio
- El servidor comprueba el número de usuario en línea, si es superior a 10
  - El nuevo usuario asume el rol de espectador, donde solo puede ver los mensajes.
  - Si es menor a 10, asigna el rol de jugador. Devuelve un id y el número de usuarios en línea.

- El cliente escribe un mensaje y lo envia presionando enter.
- El servidor recive el mensaje
- El servidor envia el mensaje a los usuarios en linea.
- Se repiten los paso 4,5,6
- Para salir del chat, usuario presiona Ctrl + C
- El servidro debe ser de asignar el rol espectador a los clientes que lleven menos de 10 minutos si escribir un mensaje.
