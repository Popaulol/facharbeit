def genererate_subfig(f: open, n: int):
    f.write(r"""     \begin{subfigure}[b]{0.25\textwidth}
         \centering
         \includegraphics[width=\textwidth]{./assets/minimax_visualisation/""" + str(n) + """}
         \caption{Schritt: """ + str(n) + """}
         \label{fig:minimax-visulisation-""" + str(n) + """}
     \end{subfigure}
     """)

if __name__ == '__main__':
    with open("visuazation_listing.txt", "w") as f:
        for i in range(1, 33):
            genererate_subfig(f, i)
