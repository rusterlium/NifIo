defmodule NifIoTest do
  use ExUnit.Case
  doctest NifIo

  test "read until end" do
    {:ok, handle} = NifIo.open("test/test_file")
    assert NifIo.read_until(handle, 0) == "first line\nsecond line"
  end

  test "read first line" do
    {:ok, handle} = NifIo.open("test/test_file")
    assert NifIo.read_until(handle, ?\n) == "first line\n"
  end
end
