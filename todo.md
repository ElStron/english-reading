Se añadira una estrcutura que obligara a todas las implementaciones a tener 
los siguientes metodos:

- Titulo del evento, para comprobar si es empty tanto en el update como loadedMessage
- Un metodo que desencadene su contenido. que sera el que maneje el task async
- Se comprobara si el titulo existe en la lista de eventos.
- por lo que en el Layout estructure deberia haber un Loading_event y un event_data
- El event_loaded deberia asignar no remplazar el  valor de event_data