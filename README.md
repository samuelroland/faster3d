# Faster3D
> A lab to optimize 3D printing time by reducing the idle movement, in FTP_Alg (algorithmic course) at MSE HES-SO 

## How to run
1. Get the data inside a `data/` folder (sorry if you don't have the .txt files I don't want to commit them)
1. `cargo run` is just going to list the available datasets with stats

    ```
    Dataset nid_d_abeille.txt
    Layers: 132
    Total segments: 121839
    Min segments/layer: 908
    Max segments/layer: 2028
    Total distance: 163344139.7

    Dataset engrenage_decentre.txt
    Layers: 42
    Total segments: 101541
    Min segments/layer: 2048
    Max segments/layer: 5473
    Total distance: 69799127.5

    Dataset de.txt
    Layers: 105
    Total segments: 36390
    Min segments/layer: 137
    Max segments/layer: 1110
    Total distance: 36095867.5

    Dataset cote_de_mailles.txt
    Layers: 25
    Total segments: 672351
    Min segments/layer: 17495
    Max segments/layer: 44342
    Total distance: 825597774.1
    ```

