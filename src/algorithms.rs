pub fn bubble_sort(arr: &mut [i32]) -> i64 {
    let mut iterations = 0;
    let len = arr.len();

    for i in 0..len {
        iterations += 1;

        // Percorre do início até o fim do array
        // diminuindo a quantidade de iterações a cada passo
        for j in 0..(len - i - 1) {
            iterations += 1;

            // Compara o elemento atual com o próximo elemento
            // Se o elemento atual for maior, realiza a troca
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }

    iterations
}

pub fn improved_bubble_sort(arr: &mut [i32]) -> i64 {
    let mut iterations = 0;
    let mut len = arr.len();
    let mut swapped = true;

    // Executa enquanto houver trocas sendo realizadas
    while swapped {
        iterations += 1;

        // Assume que não haverá mais trocas nesta iteração
        swapped = false;

        // Percorre o array da segunda posição até o último elemento
        // Começa na segunda posição pois o IF de dentro compara com um elemento antes
        for i in 1..len {
            iterations += 1;

            // Compara o elemento atual com o elemento anterior
            // Se o elemento anterior for maior, realiza a troca
            // Se não houver uma troca, significa que o array já está ordenado
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);

                // Sinaliza que uma troca foi feita nesta iteração,
                // fazendo com que o WHILE rode de novo
                swapped = true;
            }
        }

        // Diminui o tamanho do array em 1 a cada iteração,
        // pois o maior elemento já foi movido para a última posição
        len -= 1;
    }

    iterations
}

pub fn selection_sort(arr: &mut [i32]) -> i64 {
    let mut iterations = 0;

    let len = arr.len();

    for i in 0..len {
        iterations += 1;

        // Inicializa a posição mínima como sendo a posição atual
        let mut min = i;

        // Percorre o array a partir da próxima posição até o fim
        for j in (i + 1)..len {
            iterations += 1;

            // Compara o elemento atual com o menor elemento encontrado até o momento
            // Se o elemento atual for menor que o atual, atualiza a posição mínima para a posição atual
            if arr[j] < arr[min] {
                min = j;
            }
        }

        // Após percorrer o loop interno, realiza a troca do elemento na posição atual com o menor elemento encontrado
        arr.swap(i, min);
    }

    iterations
}

pub fn improved_selection_sort(arr: &mut [i32]) -> i64 {
    let mut iterations = 0;

    let mut len = arr.len();

    for i in 0..len {
        iterations += 1;

        let mut min_index = i;
        let mut max_index = i;

        // Percorre o array a partir da próxima posição até o fim
        for j in i + 1..len {
            iterations += 1;

            // Compara o elemento atual com o menor elemento encontrado até o momento
            // Se o elemento atual for menor, atualiza a posição mínima para a posição atual
            if arr[j] < arr[min_index] {
                min_index = j;
            }

            // Compara o elemento atual com o maior elemento encontrado até o momento
            // Se o elemento atual for maior, atualiza a posição máxima para a posição atual
            if arr[j] > arr[max_index] {
                max_index = j;
            }
        }

        // Verifica se a posição mínima é igual a posição atual, se sim,
        // realiza a troca entre o elemento da posição atual e o elemento da posição mínima
        if min_index != i {
            arr.swap(i, min_index);
        }

        // Verifica as possibilidades para o elemento máximo
        if max_index == i {
            max_index = min_index;
        } else if max_index == min_index {
            max_index = i;
        }

        // Verifica se a posição máxima não está na última posição, se sim,
        // realiza a troca entre o elemento da última posição e o elemento da posição máxima
        if max_index != len - 1 {
            arr.swap(len - 1, max_index);
        }

        // Diminui o valor de len em 1, pois o maior elemento já está na posição correta
        len -= 1;
    }

    // Inverte a ordem dos elementos no array
    arr.reverse();

    iterations
}

pub fn insertion_sort(arr: &mut [i32]) -> i64 {
    let mut iterations = 0;

    let len = arr.len();

    for i in 0..len {
        iterations += 1;

        let mut j = i;

        // Executa enquanto a posição atual for maior que 0 e o elemento anterior for maior que o elemento atual
        while j > 0 && arr[j - 1] > arr[j] {
            iterations += 1;

            // Realiza a troca entre o elemento atual e o elemento anterior
            arr.swap(j, j - 1);
            j -= 1;
        }
    }

    iterations
}

pub fn merge_sort(arr: &mut [i32]) -> i64 {
    do_merge_sort(arr, &mut 0)
}

fn do_merge_sort(arr: &mut [i32], iterations: &mut i64) -> i64 {
    *iterations += 1;

    let len = arr.len();

    // Caso base: se o tamanho do array for menor que 2, não é mais necessário ordenar
    if len < 2 {
        return *iterations;
    }

    // Armazena a posição da metade do array
    let mid = len / 2;

    // Chama recursivamente a função do_merge_sort para ordenar a metade esquerda do array
    do_merge_sort(&mut arr[..mid], iterations);

    // Chama recursivamente a função do_merge_sort para ordenar a metade direita do array
    do_merge_sort(&mut arr[mid..], iterations);

    // Cria um novo vetor chamado "merged" com capacidade já definida sendo igual ao tamanho do array
    let mut merged = Vec::with_capacity(len);
    let mut i = 0;
    let mut j = mid;

    // Mescla as duas metades ordenadas do array em um único vetor "merged"
    while i < mid && j < len {
        *iterations += 1;

        // Compara os elementos das duas metades e adiciona o menor elemento em "merged"
        if arr[i] <= arr[j] {
            merged.push(arr[i]);
            i += 1;
        } else {
            merged.push(arr[j]);
            j += 1;
        }
    }

    // Adiciona os elementos restantes da metade esquerda em "merged"
    while i < mid {
        *iterations += 1;

        merged.push(arr[i]);
        i += 1;
    }

    // Adiciona os elementos restantes da metade direita em "merged"
    while j < len {
        *iterations += 1;

        merged.push(arr[j]);
        j += 1;
    }

    // Copia os elementos de "merged" de volta para o array original
    arr.copy_from_slice(&merged[..]);

    *iterations
}

pub fn heap_sort(arr: &mut [i32]) -> i64 {
    let mut iterations = 0;

    // Constrói um heap máximo a partir do array
    build_max_heap(arr, &mut iterations);

    let len = arr.len();

    // Ordena o array removendo repetidamente o elemento máximo e mantendo o heap
    for i in (1..len).rev() {
        // Troca o elemento máximo (raiz) com o último elemento não ordenado
        arr.swap(0, i);

        // Restaura a propriedade de heap máximo no subarray não ordenado
        max_heapify(&mut arr[..i], 0, &mut iterations);
    }

    iterations
}

fn build_max_heap(arr: &mut [i32], iterations: &mut i64) {
    let len = arr.len();
    let mid = len / 2;

    // Percorre os elementos a partir da metade do array até o início, chamando max_heapify em cada elemento
    for i in (0..mid).rev() {
        *iterations += 1;

        max_heapify(arr, i, iterations);
    }
}

fn max_heapify(arr: &mut [i32], mut i: usize, iterations: &mut i64) {
    let len = arr.len();

    loop {
        *iterations += 1;

        let mut largest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        // Compara o elemento atual com seus filhos e encontra o maior elemento
        if left < len && arr[left] > arr[largest] {
            largest = left;
        }

        if right < len && arr[right] > arr[largest] {
            largest = right;
        }

        // Se o elemento atual for o maior, o heap já está corretamente organizado e o loop é interrompido
        if largest == i {
            break;
        }

        // Caso contrário, troca o elemento atual com o maior filho e atualiza a posição do elemento atual
        arr.swap(i, largest);
        i = largest;
    }
}

pub fn quick_sort(arr: &mut [i32]) -> i64 {
    let mut iterations = 0;

    do_quick_sort(arr, &mut iterations);

    iterations
}
pub fn do_quick_sort(arr: &mut [i32], iterations: &mut i64) {
    *iterations += 1;

    let len = arr.len();

    // Caso base: se o tamanho do array for menor que 2, não é mais necessário ordenar
    if len < 2 {
        return;
    }

    // Realiza a partição do array e obtém o índice do pivô
    let pivot_index = partition(arr, iterations);

    // Chama recursivamente a função quick_sort para as subpartes do array
    do_quick_sort(&mut arr[..pivot_index], iterations);
    do_quick_sort(&mut arr[pivot_index + 1..len], iterations);
}

fn partition(arr: &mut [i32], iterations: &mut i64) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;

    // Troca o pivô com o último elemento do array
    arr.swap(pivot_index, len - 1);

    let mut i = 0;

    // Percorre o array, exceto o último elemento (pivô)
    for j in 0..len - 1 {
        *iterations += 1;

        // Se o elemento atual for menor ou igual ao pivô, troca-o com a posição atual de i
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }

    // Troca o pivô para a posição correta no array
    arr.swap(i, len - 1);

    i
}

#[cfg(test)]
mod tests {
    use rand::{seq::SliceRandom, thread_rng};

    const SIZE: i32 = 10;
    const SORTED_ARR: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    #[test]
    fn bubble_sort() {
        let mut arr = shuffled_arr();
        super::bubble_sort(&mut arr);

        assert_eq!(arr, SORTED_ARR);
    }

    #[test]
    fn improved_bubble_sort() {
        let mut arr = shuffled_arr();
        super::improved_bubble_sort(&mut arr);

        assert_eq!(arr, SORTED_ARR);
    }

    #[test]
    fn selection_sort() {
        let mut arr = shuffled_arr();
        super::selection_sort(&mut arr);

        assert_eq!(arr, SORTED_ARR);
    }

    #[test]
    fn improved_selection_sort() {
        let mut arr = shuffled_arr();
        super::improved_selection_sort(&mut arr);

        assert_eq!(arr, SORTED_ARR);
    }

    #[test]
    fn insertion_sort() {
        let mut arr = shuffled_arr();
        super::insertion_sort(&mut arr);

        assert_eq!(arr, SORTED_ARR);
    }

    #[test]
    fn merge_sort() {
        let mut arr = shuffled_arr();
        super::merge_sort(&mut arr);

        assert_eq!(arr, SORTED_ARR);
    }

    #[test]
    fn heap_sort() {
        let mut arr = shuffled_arr();
        super::heap_sort(&mut arr);

        assert_eq!(arr, SORTED_ARR);
    }

    #[test]
    fn quick_sort() {
        let mut arr = shuffled_arr();
        super::quick_sort(&mut arr);

        assert_eq!(arr, SORTED_ARR);
    }

    fn shuffled_arr() -> Vec<i32> {
        let mut arr: Vec<_> = (0..SIZE).collect();

        arr.shuffle(&mut thread_rng());

        arr
    }
}