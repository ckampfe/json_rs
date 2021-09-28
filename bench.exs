very_small_json = "{\"abc\":1,\"def\":true,\"ghi\":[]}"
small_json = File.read!("example_small.json")
large_json = File.read!("example_large.json")
issue_90_json = File.read!("issue-90.json")

very_small_term = very_small_json |> Jason.decode!()
small_term = small_json |> Jason.decode!()
large_term = large_json |> Jason.decode!()
issue_90_term = issue_90_json |> Jason.decode!()

Benchee.run(
  %{
    "JsonRs encode/1" => fn term ->
      JsonRs.encode(term)
    end,
    "Jason encode/1" => fn term ->
      Jason.encode(term)
    end
  },
  inputs: %{
    "very small" => very_small_term,
    "33kB" => small_term,
    "100kB" => large_term,
    "issue-90" => issue_90_term
  }
)

Benchee.run(
  %{
    "JsonRs decode/1" => fn term ->
      JsonRs.decode(term)
    end,
    "Jason decode/1" => fn term ->
      Jason.decode(term)
    end
  },
  inputs: %{
    "very small" => very_small_json,
    "33kB" => small_json,
    "100kB" => large_json,
    "issue-90" => issue_90_json
  }
)
