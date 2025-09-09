#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

// Helper function to find partition point (equivalent to Rust's partition_point)
size_t partition_point(const unsigned int *arr, size_t len, unsigned int val, bool less_than) {
    size_t left = 0, right = len;
    while (left < right) {
        size_t mid = left + (right - left) / 2;
        if (less_than ? arr[mid] < val : arr[mid] <= val) {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    return left;
}

// Helper function for saturating subtraction
unsigned int saturating_sub(unsigned int a, unsigned int b) {
    return (a > b) ? (a - b) : 0;
}

// Comparison function for qsort
int compare_uint(const void *a, const void *b) {
    unsigned int ua = *(const unsigned int*)a;
    unsigned int ub = *(const unsigned int*)b;
    if (ua < ub) return -1;
    if (ua > ub) return 1;
    return 0;
}

size_t num_exploded_bales(const unsigned int *bale_positions, size_t n, size_t starting_index) {
    // Create seen array (equivalent to Rust's Vec<bool>)
    bool *seen = calloc(n, sizeof(bool));
    if (!seen) {
        fprintf(stderr, "Memory allocation failed\n");
        exit(1);
    }

    // Create frontier arrays (we'll use two arrays and swap between them)
    size_t *frontier = malloc(n * sizeof(size_t));
    size_t *next_frontier = malloc(n * sizeof(size_t));
    if (!frontier || !next_frontier) {
        fprintf(stderr, "Memory allocation failed\n");
        exit(1);
    }

    // Initialize frontier with starting_index
    frontier[0] = starting_index;
    size_t frontier_len = 1;
    seen[starting_index] = true;

    unsigned int t = 1;

    while (frontier_len > 0) {
        size_t next_frontier_len = 0;

        // Process each element in current frontier
        for (size_t f_idx = 0; f_idx < frontier_len; f_idx++) {
            size_t i = frontier[f_idx];

            // Calculate bounds
            unsigned int target_lower = saturating_sub(bale_positions[i], t);
            unsigned int target_upper = bale_positions[i] + t;

            size_t lower_bound = partition_point(bale_positions, n, target_lower, true);
            size_t upper_bound = partition_point(bale_positions, n, target_upper, false);

            // Add unseen indices in range to next_frontier
            for (size_t idx = lower_bound; idx < upper_bound; idx++) {
                if (!seen[idx]) {
                    next_frontier[next_frontier_len++] = idx;
                    seen[idx] = true;
                }
            }
        }

        // Swap frontiers
        size_t *temp = frontier;
        frontier = next_frontier;
        next_frontier = temp;
        frontier_len = next_frontier_len;

        t++;
    }

    // Count true values in seen array
    size_t count = 0;
    for (size_t i = 0; i < n; i++) {
        if (seen[i]) {
            count++;
        }
    }

    free(seen);
    free(frontier);
    free(next_frontier);

    return count;
}

int solve(FILE *input, FILE *output) {
    size_t n;
    if (fscanf(input, "%zu", &n) != 1) {
        return -1;
    }

    unsigned int *bale_positions = malloc(n * sizeof(unsigned int));
    if (!bale_positions) {
        fprintf(stderr, "Memory allocation failed\n");
        return -1;
    }

    // Read bale positions
    for (size_t i = 0; i < n; i++) {
        if (fscanf(input, "%u", &bale_positions[i]) != 1) {
            free(bale_positions);
            return -1;
        }
    }

    // Sort bale positions using qsort
    qsort(bale_positions, n, sizeof(unsigned int), compare_uint);

    // Find maximum exploded bales
    size_t max_exploded = 0;
    for (size_t i = 0; i < n; i++) {
        size_t exploded = num_exploded_bales(bale_positions, n, i);
        if (exploded > max_exploded) {
            max_exploded = exploded;
        }
    }

    fprintf(output, "%zu\n", max_exploded);

    free(bale_positions);
    return 0;
}

void run_problem() {
    FILE *input = fopen("angry.in", "r");
    FILE *output = fopen("angry.out", "w");

    // Fallback to stdin/stdout if files don't exist
    if (!input) {
        input = stdin;
    }
    if (!output) {
        output = stdout;
    }

    if (solve(input, output) != 0) {
        fprintf(stderr, "Error solving problem\n");
        exit(1);
    }

    if (input != stdin) {
        fclose(input);
    }
    if (output != stdout) {
        fclose(output);
    }
}

int main() {
    run_problem();
    return 0;
}
