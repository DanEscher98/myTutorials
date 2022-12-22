defmodule TodoApp.BucketTest do
  # is responsible for setting up the module for testing
  use ExUnit.Case, async: true

  test "stores values by key" do
    {:ok, bucket} = TodoApp.Bucket.start_link([])
    assert TodoApp.Bucket.get(bucket, "milk") == nil

    TodoApp.Bucket.put(bucket, "milk", 3)
    assert TodoApp.Bucket.get(bucket, "milk") == 3
  end
end
