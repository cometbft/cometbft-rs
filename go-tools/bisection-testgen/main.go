package main

import "github.com/cometbft/cometbft-rs/go-tools/bisection-testgen/internal/generator"

const val_list_json_path = "./tests/json/val_list.json"

func main() {
	generator.GenerateManyHeaderBisectionCases(val_list_json_path)
	generator.GenerateSingleStepSkippingCases(val_list_json_path)
	generator.GenerateSingleStepSequentialCases(val_list_json_path)
}
