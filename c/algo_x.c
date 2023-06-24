
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <time.h>

//#define N 16
//#define K 23

#define N 5
#define K 5

//#define N 3
//#define K 4

#define N_COL 2*N*N+K*2*N*N
#define N_ROW 4*N*N*N*N

struct List
{
    int data;
    struct List * next;
};

struct Node
{
    int piece;
    int emplacement;
    int rotation;

    int row;
    int col;
    int count;

    struct Node * U;
    struct Node * D;
    struct Node * E;
    struct Node * O;

    struct Node * head;
};


struct Node * columns[N_COL+1];
int dropped_rows[N_ROW];



void addNodeC(struct Node * node, struct Node * new_node)
{
    new_node->D = node->D;
    new_node->U = node;

    node->D->U = new_node;
    node->D = new_node;
}

void addNodeR(struct Node * node, struct Node * new_node)
{
    new_node->E = node->E;
    new_node->O = node;

    node->E->O = new_node;
    node->E = new_node;
}

void addLL(struct List * start, struct List * end)
{
    //printf("  mmmh %i\n", start->data);
    if (start->next == NULL)
    {
        start->next = end;
    }
    else
    {
        addLL(start->next, end);
    }
}

int popLL(struct List * start)
{
    if (start->next != NULL)
    {
        if (start->next->next == NULL)
        {
            struct List * ptr = start->next;
            start->next = NULL;
            int data = ptr->data;
            free(ptr);
            return data;
        }
        else
        {
            return popLL(start->next);
        }
    }
}

void coverCol(struct Node * col)
{
    col->E->O = col->O;
    col->O->E = col->E;

    for (struct Node * node_v=col->D; node_v!=col; node_v=node_v->D)
    {
        for (struct Node * node_h=node_v->E; node_h!=node_v; node_h=node_h->E)
        {
            node_h->D->U = node_h->U;
            node_h->U->D = node_h->D;

            node_h->head->count--;
        }
    }
}

void uncoverCol(struct Node * col)
{

    for (struct Node * node_v=col->U; node_v!=col; node_v=node_v->U)
    {
        for (struct Node * node_h=node_v->O; node_h!=node_v; node_h=node_h->O)
        {
            node_h->D->U = node_h;
            node_h->U->D = node_h;

            node_h->head->count++;
        }
    }
    col->E->O = col;
    col->O->E = col;
}

int solve(struct Node * init_ptr, struct List * output, int rec)
{
    struct Node * ptr_dbll=init_ptr, * min_col, * last_node;
    struct List * tmp;
    
    if (ptr_dbll->E == ptr_dbll)
    {
        return 1;
    }
    min_col = ptr_dbll->E;
    
    for (struct Node * node=min_col->E; node!=ptr_dbll; node=node->E)
    {
        if (node->count < min_col->count)
            min_col = node;
    }
    coverCol(min_col);

    for (struct Node * node_v=min_col->D; node_v!=min_col; node_v=node_v->D)
    {
        for (struct Node * node_h=node_v->E; node_h!=node_v; node_h=node_h->E)
        {
            coverCol(node_h->head);
        }
        if (solve(init_ptr, output, rec+1))
        {
            tmp = (struct List *) malloc(sizeof(struct List));
            tmp->data = node_v->row;
            tmp->next;
            addLL(output, tmp);
            return 1;
        }
        for (struct Node * node_h=node_v->O; node_h!=node_v; node_h=node_h->O)
        {
            uncoverCol(node_h->head);
        }
    }

    uncoverCol(min_col);

    return 0;
}


int * generate_puzzle()
{

    int * puzzle = (int *) malloc(sizeof(int)*N*N*4), rand_col;
    for (int i=0; i<N; i++)
    {
        for (int j=0; j<N; j++)
        {
            rand_col = rand()%K;
            puzzle[4*(j*N+i)+3] = rand_col;
            puzzle[4*(j*N+(i+1)%N)+1] = rand_col;
            
            rand_col = rand()%K;
            puzzle[4*(j*N+i)+1] = rand_col;
            puzzle[4*(j*N+(i+N-1)%N)+3] = rand_col;
            
            rand_col = rand()%K;
            puzzle[4*(j*N+i)+2] = rand_col;
            puzzle[4*(((j+1)%N)*N+i)+0] = rand_col;
            
            rand_col = rand()%K;
            puzzle[4*(j*N+i)+0] = rand_col;
            puzzle[4*(((j+N-1)%N)*N+i)+2] = rand_col;
        }
    }
    return puzzle;
}

int * shuffle_puzzle(int * puzzle)
{
    
    int r1, r2, swap;

    for (int i=0; i<500; i++)
    {
        if (rand()%2)
        { // swap
            r1 = rand()%(N*N);
            r2 = rand()%(N*N);

            for (int d=0; d<4; d++)
            {
                swap = puzzle[4*r1+d];
                puzzle[4*r1+d] = puzzle[4*r2+d];
                puzzle[4*r2+d] = swap;
            }
        }
        else
        { //rotate
            r1 = rand()%(N*N);
            r2 = rand()%4;

            for (int d=0; d<4; d++)
            {
                swap = puzzle[4*r1+d];
                puzzle[4*r1+d] = puzzle[4*r1+(d+r2)%4];
                puzzle[4*r1+(d+r2)%4] = swap;
            }
            
        }
    }

    return puzzle;
}

void print_puzzle(int * puzzle)
{
    
    for (int i=0; i<N; i++)
    {
        for (int j=0; j<N; j++)
            printf("  /%i\\  |", puzzle[4*(j*N+i)+1]);
        printf("\n");
        for (int j=0; j<N; j++)
            printf(" |%i %i| |", puzzle[4*(j*N+i)+0], puzzle[4*(j*N+i)+2]);
        printf("\n");
        for (int j=0; j<N; j++)
            printf("  \\%i/  |", puzzle[4*(j*N+i)+3]);
        printf("\n");
        for (int j=0; j<N; j++)
            printf("-------|");
        printf("\n");
    }
}


struct Node * init_exact_cover_problem(int * puzzle)
{

    struct Node * init_ptr, * ptr, * col_node;
    struct List * ref, * tmp, * to_add;
    int piece, emplacement, col, i_e, j_e, piece_N, piece_E, piece_S, piece_O;
    int i_string[N_COL];

    init_ptr = (struct Node *) malloc(sizeof(struct Node)); //&(struct Node) {-1, -1, -1, -1, 0, 0, NULL, NULL, NULL, NULL, NULL};
    init_ptr->col = -1;
    init_ptr->count = 0;
    init_ptr->U = init_ptr;
    init_ptr->D = init_ptr;
    init_ptr->E = init_ptr;
    init_ptr->O = init_ptr;
    init_ptr->head = init_ptr;

    columns[0] = init_ptr;

    for (int i=0; i<N_COL; i++)
    { // On initialise la liste des colonnes (pour chercher la colonne qui possède le moins de contrainte)

        col_node = (struct Node *) malloc(sizeof(struct Node));
        col_node->col = i;

        col_node->count = 0;

        col_node->U = col_node;
        col_node->D = col_node;

        col_node->O = init_ptr->O;
        col_node->E = init_ptr;

        init_ptr->O->E = col_node;
        init_ptr->O = col_node;

        col_node->head = col_node;

        columns[i+1] = col_node;

    }

    
    for (int rot=0; rot<4; rot++)
    { // On représente le problème sous forme de Dancing Links
        for (int row=0; row<N*N*N*N; row++)
        {
            piece = row/(N*N);
            emplacement = row%(N*N);

            
            to_add = (struct List *) malloc(sizeof(struct List));
            to_add->data = piece;
            to_add->next = NULL;
            ref = to_add;

            tmp = (struct List *) malloc(sizeof(struct List));
            tmp->data = N*N + emplacement;
            tmp->next = NULL;
            addLL(ref, tmp);


            i_e = emplacement/N;
            j_e = emplacement%N;

            piece_N = puzzle[4*piece+(0+rot)%4];
            piece_E = puzzle[4*piece+(1+rot)%4];
            piece_S = puzzle[4*piece+(2+rot)%4];
            piece_O = puzzle[4*piece+(3+rot)%4];

            tmp = (struct List *) malloc(sizeof(struct List));
            tmp->data = N*N + N*N + K*(i_e*N+j_e) + piece_N;
            tmp->next = NULL;
            addLL(ref, tmp);

            for (int i=0; i<K; i++)
            {
                if (i != piece_S)
                {

                    tmp = (struct List *) malloc(sizeof(struct List));
                    tmp->data = N*N + N*N + K*((i_e*N+j_e+N)%(N*N)) + i;
                    tmp->next = NULL;
                    addLL(ref, tmp);

                }
            }

            tmp = (struct List *) malloc(sizeof(struct List));
            tmp->data = N*N + N*N + K*(N*N) + K*(j_e*N+i_e) + piece_E;
            tmp->next = NULL;
            addLL(ref, tmp);

            for (int i=0; i<K; i++)
            {
                if (i != piece_O)
                {
                    tmp = (struct List *) malloc(sizeof(struct List));
                    tmp->data = N*N + N*N + K*(N*N) + K*((j_e*N+i_e+N)%(N*N)) + i;
                    tmp->next = NULL;
                    addLL(ref, tmp);
                }
            }

            for (int i=0; i<N_COL; i++)
            {
                i_string[i] = 0;
            }

            struct Node * old_ptr=NULL;
            
            while (ref != NULL)
            {

                col = ref->data;
                i_string[col] = 1;

                ptr = (struct Node *) malloc(sizeof(struct Node));
                ptr->piece = piece;
                ptr->emplacement = emplacement;
                ptr->rotation = rot;
                ptr->row = row + rot*N*N*N*N;
                ptr->col = col;
                ptr->D = ptr;
                ptr->U = ptr;
                ptr->E = ptr;
                ptr->O = ptr;
                ptr->head = columns[col+1];

                addNodeC(columns[col+1], ptr);
                columns[col+1]->count++;

                if (old_ptr != NULL)
                {
                    addNodeR(old_ptr, ptr);
                }

                ref = ref->next;
                old_ptr = ptr;
            }

        }

    }

    return init_ptr;
}

int main()
{

    srand(time(NULL));

    int * puzzle, r;

    struct Node * init_ptr;
    struct List * output;
    time_t begin, end;
    unsigned long secondes;

    printf("\n");
    
    for (int i=0; i<10; i++)
    {
        puzzle = shuffle_puzzle(generate_puzzle());
        init_ptr = init_exact_cover_problem(puzzle);
        output = &(struct List) {-1, NULL};

        begin = time( NULL );
        r = solve(init_ptr, output, 0);
        end = time( NULL);

        secondes = (unsigned long) difftime( end, begin );
        printf("%ld; ", secondes ); 

        free(puzzle);
    }
    printf("\n");

    

    //printf("Finished in %ld sec\n", secondes ); 


    /*
    int row, rotation, piece, emplacement;
    int * puzzle_solution = (int *) malloc(sizeof(int) * N * N * 4);
    output = output->next;
    while (output != NULL)
    {

        row = output->data;

        rotation = row/(N*N*N*N);
        piece = (row%(N*N*N*N))/(N*N);
        emplacement = (row%(N*N*N*N))%(N*N);

        printf("Rotation : %i, piece : %i, emplacement : %i\n", rotation, piece, emplacement);

        for (int i=0; i<4; i++)
        {
            puzzle_solution[4*emplacement+i] = puzzle[4*piece+(i+rotation)%4];
        }

        printf("%i \n", output->data);
        output = output->next;
    }

    print_puzzle(puzzle_solution);
    free(puzzle_solution);
    */

    
    return 0;
}

