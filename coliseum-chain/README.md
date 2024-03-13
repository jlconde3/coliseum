# ColiseumCoin

## Proceso de Creación de RustyCoin Blockchain

En el inicio de la RustyCoin Blockchain, se genera una instancia inicializando el bloque génesis, que actúa como el punto de partida para la cadena de bloques. Este bloque contiene información fundamental sobre la configuración inicial de la blockchain.

### Sistema de Comunicación para la Alta/Baja de Nodos

1. **Conexión de un Nuevo Nodo:**

   - Cuando un nuevo nodo se incorpora a la red de RustyCoin, se conecta a nodos existentes para establecer una comunicación efectiva.

2. **Recepción de la Lista de Nodos:**

   - El nuevo nodo recibe una lista completa de todos los nodos en la red, permitiéndole establecer conexiones con sus pares.

3. **Envío de Dirección IP:**

   - La nueva adición envía su dirección IP a los demás nodos, facilitando la creación de conexiones bidireccionales.

4. **Contacto con Nodos para Obtener Cadena Principal:**

   - El nodo recién incorporado se comunica con nodos representativos para obtener detalles sobre la cadena de bloques actual. Esto incluye solicitar el hash del último bloque y el número total de bloques en la cadena.

5. **Elección de la Cadena Mayoritaria:**

   - Comparando la información obtenida, el nodo selecciona la cadena más larga o con el hash más reciente, garantizando la adopción de la cadena respaldada por la mayoría de la red.

6. **Recepción de la Cadena de Bloques Elegida:**
   - El nuevo nodo integra la cadena de bloques seleccionada a su copia local.

### Tipos de Nodos

1. **Minadores:**

- Los nodos minadores reciben transacciones pendientes y resuelven problemas criptográficos específicos para cada blockchain, proponiendo la solución a la red una vez encontrada.

2. **Traders:**

- Identifican a los usuarios mediante un UUID único.
- Verifican la validez de las transacciones, asegurándose de que el emisor tenga fondos suficientes.
- Distribuyen la transacción a través de la red para conocimiento general.

3. **Minador-Trader:**
   - Combina funciones de minado y trading, participando en ambos procesos.

### Transacciones

1. **Distribución de Transacciones:**

   - Cada nueva transacción se difunde a toda la red para conocimiento y procesamiento.

2. **Verificación de Fondos:**

   - Previo a aceptar una transacción, los nodos verifican la disponibilidad de fondos del emisor.

3. **Inclusión en la Lista de Transacciones:**

   - Las transacciones validadas se incluyen en la lista pendiente de cada nodo.

4. **Aprobación por el 51%:**
   - Para ser considerada válida, una transacción debe contar con la aprobación de al menos el 51% de los nodos en la red.

### Bloques

1. **Generación del Bloque:**

   - Cuando hay suficientes transacciones pendientes, un nodo minador inicia la generación de un nuevo bloque.

2. **Resolución del Problema Criptográfico:**

   - Inicia el cálculo de la solución al problema criptográfico asociado al bloque, asegurando la prueba de trabajo.

3. **Comunicación de la Solución:**

   - La solución se comunica al resto de los nodos.

4. **Validación por el 51%:**

   - Los nodos validan la solución propuesta, requiriendo la aprobación del 51% para aceptar el bloque.

5. **Recopilación de Transacciones:**

   - Se recopilan transacciones pendientes desde el bloque anterior hasta el momento actual.

6. **Inclusión en el Nuevo Bloque:**

   - Las transacciones se añaden al nuevo bloque junto con la última transacción que contiene la recompensa para el minador.

7. **Difusión del Nuevo Bloque:**
   - El nuevo bloque se difunde para que los nodos lo incorporen a sus copias locales.
