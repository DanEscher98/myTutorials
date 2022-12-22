defmodule TodoApp.RegistryTest do
  use ExUnit.Case, async: true

  setup do
    registry = start_supervised!(TodoApp.Registry)
    %{registry: registry}
  end

  test "spawns buckets", %{registry: registry} do
    assert TodoApp.Registry.lookup(registry, "shopping") == :error

    TodoApp.Registry.create(registry, "shopping")
    assert {:ok, bucket} = TodoApp.Registry.lookup(registry, "shopping")

    TodoApp.Bucket.put(bucket, "milk", 1)
    assert TodoApp.Bucket.get(bucket, "milk") == 1
  end

  test "removes buckets on exit", %{registry: registry} do
    TodoApp.Registry.create(registry, "shopping")
    {:ok, bucket} = TodoApp.Registry.lookup(registry, "shopping")
    Agent.stop(bucket)
    assert TodoApp.Registry.lookup(registry, "shopping") == :error
  end
end
