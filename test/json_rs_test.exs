defmodule JsonRsTest do
  use ExUnit.Case, async: true
  doctest JsonRs

  test "encode/1 bools" do
    assert {:ok, "true"} = JsonRs.encode(true)
    assert {:ok, "false"} = JsonRs.encode(false)
  end

  test "encode/1 nil" do
    assert {:ok, "null"} = JsonRs.encode(nil)
  end

  test "encode/1 strings" do
    assert {:ok, "\"hi\""} = JsonRs.encode("hi")
    assert {:ok, "\"\""} = JsonRs.encode("")
  end

  test "encode/1 atoms" do
    assert {:ok, "\"ok\""} = JsonRs.encode(:ok)
  end

  test "encode/1 lists" do
    assert {:ok, "[]"} = JsonRs.encode([])
    assert {:ok, "[1,\"a\",true,\"ok\"]"} = JsonRs.encode([1, "a", true, :ok])
  end

  test "encode/1 maps" do
    assert {:ok, "{}"} = JsonRs.encode(%{})
    assert {:ok, "{\"a\":1}"} = JsonRs.encode(%{a: 1})
    assert {:ok, "{\"a\":1}"} = JsonRs.encode(%{"a" => 1})
    assert {:ok, "{\"b\":2,\"a\":1}"} = JsonRs.encode(%{"a" => 1, b: 2})

    assert {:ok, _} =
             1..1000
             |> Enum.map(fn n -> {"#{n}", "#{n}#{n}"} end)
             |> Enum.into(%{})
             |> JsonRs.encode()
  end

  test "encode/1 integers" do
    assert {:ok, "123"} = JsonRs.encode(123)

    assert {:ok, "82390847923847928374928374928374928374"} =
             JsonRs.encode(82_390_847_923_847_928_374_928_374_928_374_928_374)
  end

  test "encode/1 floats" do
    assert {:ok, "123.4"} = JsonRs.encode(123.4)
    assert {:ok, "14000000000.0"} = JsonRs.encode(1.4e10)
    assert {:ok, "1.4e23"} = JsonRs.encode(1.4e23)
  end

  test "issue-90" do
    issue_90 = File.read!("issue-90.json") |> Jason.decode!()
    assert {:ok, _} = JsonRs.encode(issue_90)
  end

  test "decode/1 null" do
    assert {:ok, nil} == JsonRs.decode("null")
  end

  test "decode/1 bool" do
    assert {:ok, true} == JsonRs.decode("true")
    assert {:ok, false} == JsonRs.decode("false")
  end

  test "decode/1 number" do
    assert {:ok, 40} == JsonRs.decode("40")
    assert {:ok, -40} == JsonRs.decode("-40")
    assert {:ok, 40.0} == JsonRs.decode("40.0")
    assert {:ok, -40.0} == JsonRs.decode("-40.0")
  end

  test "decode/1 string" do
    assert {:ok, "some string"} == JsonRs.decode("\"some string\"")
  end

  test "decode/1 array" do
    assert {:ok, [1, 2, 3, "something", true, false, nil]} ==
             JsonRs.decode("[1,2,3,\"something\",true,false,null]")
  end

  test "decode/1 object" do
    assert {:ok, %{}} == JsonRs.decode("{}")

    assert {:ok,
            %{
              "abc" => 1,
              "def" => true,
              "ghi" => []
            }} == JsonRs.decode("{\"abc\":1,\"def\":true,\"ghi\":[]}")
  end

  test "matches Jason" do
    issue_90_json = File.read!("issue-90.json")
    assert {:ok, _} = JsonRs.decode(issue_90_json)
    assert JsonRs.decode(issue_90_json) == Jason.decode(issue_90_json)
  end
end
