#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Define a struct to hold the parsed arguments
struct Arguments {
    char *input_file;
    char *output_file;
    int verbose;
};

// Function to print the usage of the program
void print_usage(const char *program_name) {
    printf("Usage: %s -i <input_file> -o <output_file> [-v]\n", program_name);
}

// Function to parse command-line arguments into the struct
void parse_arguments(int argc, char *argv[], struct Arguments *args) {
    // Initialize struct members
    args->input_file = NULL;
    args->output_file = NULL;
    args->verbose = 0;

    // Parse command-line arguments
    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "-i") == 0 && i + 1 < argc) {
            args->input_file = argv[i + 1];
            i++;
        } else if (strcmp(argv[i], "-o") == 0 && i + 1 < argc) {
            args->output_file = argv[i + 1];
            i++;
        } else if (strcmp(argv[i], "-v") == 0) {
            args->verbose = 1;
        }
    }
}

int main(int argc, char *argv[]) {
    // Define a struct to hold the parsed arguments
    struct Arguments args;

    // Parse command-line arguments into the struct
    parse_arguments(argc, argv, &args);

    // Check if required arguments are provided
    if (args.input_file == NULL || args.output_file == NULL) {
        print_usage(argv[0]);
        return 1;
    }

    // Print parsed arguments
    printf("Input file: %s\n", args.input_file);
    printf("Output file: %s\n", args.output_file);
    printf("Verbose mode: %s\n", args.verbose ? "Enabled" : "Disabled");

    // Further processing based on parsed arguments can be done here

    return 0;
}
