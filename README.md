# MetaSort

Programa para organizar archivos con la metadata de estos. 

## Descripción

Este programa es un sistema de monitorización en tiempo real que observa un directorio específico para detectar nuevos archivos o modificaciones en los existentes. Al identificar cambios, realiza acciones inmediatas para reorganizar el directorio según parámetros predefinidos. Esto incluye modificar la metadata del archivo para agregar la fecha de creación, renombrarlo según su hora de creación y priorizar el procesamiento de múltiples archivos simultáneos según criterios establecidos. El programa asegura un tiempo de procesamiento consistente, independientemente del tamaño del archivo, y maneja errores, como metadata corrupta, registrando el problema y continuando con la monitorización.

## Instalacion 

Descarga el zip file del aparado de [releases](https://github.com/Monono13/MetaSort/releases)

## Ejecución

Una vez descargado el zip ejecuta el programa compilado escribiendo en una terminal:

```bash
$ ./FileSorts
```
## Uso

Después de ejecutar el programa este le pedirá que escriba el nombre del directorio que desea escanear para realizar los cambios.

**CAUTION**

Todos los archivos serán renombrados y podría ocasionar problemas para archivos importantes con otras características, usar de manera segura al momento de asignar el directorio que se desea escanear. 

Después de que este termine de escanear se puede cerrar la terminal para desactivar el programa o presionar “**control-c**” para cancelar la ejecución. 

## Historial de Versiones

* v1.0
> Initial Release


## Licencia

[Licencia]()
