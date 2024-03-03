import tkinter as tk

COLISEUM_LOGO = r"""
 ________  ________  ___       ___  ________  _______   ___  ___  _____ ______
|\   ____\|\   __  \|\  \     |\  \|\   ____\|\  ___ \ |\  \|\  \|\   _ \  _   \
\ \  \___|\ \  \|\  \ \  \    \ \  \ \  \___|\ \   __/|\ \  \\\  \ \  \\\__\ \  \
 \ \  \    \ \  \\\  \ \  \    \ \  \ \_____  \ \  \_|/_\ \  \\\  \ \  \\|__| \  \
  \ \  \____\ \  \\\  \ \  \____\ \  \|____|\  \ \  \_|\ \ \  \\\  \ \  \    \ \  \
   \ \_______\ \_______\ \_______\ \__\____\_\  \ \_______\ \_______\ \__\    \ \__\
    \|_______|\|_______|\|_______|\|__|\_________\|_______|\|_______|\|__|     \|__|
                                      \|_________|
"""

class TerminalApp:
    def __init__(self, root):
        self.root = root
        self.root.title("Terminal App")
        self.root.configure(bg="black")

        # Área de visualización de mensajes con el logotipo
        self.text_widget = tk.Text(root, state=tk.DISABLED, border=-2, relief="flat", bg="black", fg="white", highlightthickness=0)
        self.text_widget.grid(row=0, column=0, sticky=tk.NSEW, padx=5, pady=5)


        # Muestra el mensaje en la zona de visualización
        self.text_widget.config(state=tk.NORMAL)
        self.text_widget.insert(tk.END, COLISEUM_LOGO)
        self.text_widget.insert(tk.END, "\n")
        self.text_widget.config(state=tk.DISABLED)


         # Área de entrada de texto con texto predeterminado ">>>"
        self.input_entry = tk.Entry(root, border=-2, relief="flat", bg="black", fg="white", highlightthickness=0, insertbackground="white")
        self.input_entry.grid(row=1, column=0, sticky=tk.EW, padx=5, pady=5)
        self.input_entry.insert(tk.END, ">>> ")
        self.input_entry.bind("<Return>", self.process_input)

        # Configuración de la geometría de la ventana
        self.root.grid_rowconfigure(0, weight=1)
        self.root.grid_rowconfigure(1, weight=0)
        self.root.grid_columnconfigure(0, weight=1)

        self.input_entry.focus_set()

    def process_input(self, event):
        # Obtiene el texto de la entrada
        input_text = self.input_entry.get()

        # Limpia la entrada después de presionar Enter
        self.input_entry.delete(0, tk.END)
        self.input_entry.insert(tk.END, ">>> ")

        # Muestra el mensaje en la zona de visualización
        self.text_widget.config(state=tk.NORMAL)
        self.text_widget.insert(tk.END, f"{input_text}\n")
        self.text_widget.config(state=tk.DISABLED)

        # Desplaza automáticamente hacia abajo para mostrar el mensaje más reciente
        self.text_widget.yview(tk.END)

if __name__ == "__main__":
    root = tk.Tk()
    app = TerminalApp(root)
    root.mainloop()
