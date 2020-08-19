# How to UI

Each window hosts a widget. There are many widgets. They all implement the Widget trait. UI runs the main loop:

1. Events from the system are passed down to widget.handle()
2. If the widget needs to be drawn, call widget.draw()

Widget drawing also creates a new vertexbuffer where needed. The reason is that some widgets do not fit in uber shader mode, and separating those who are and aren't is a hassle (and it's not yet clear whether or not it is indeed faster to chunk everything together into one vertexbuffer).