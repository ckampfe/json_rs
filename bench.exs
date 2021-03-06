small = File.read!("example_small.json") |> Jason.decode!()
large = File.read!("example_large.json") |> Jason.decode!()
issue_90 = File.read!("issue-90.json") |> Jason.decode!()

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
    "33kB" => small,
    "100kB" => large,
    "issue-90" => issue_90
  }
)
