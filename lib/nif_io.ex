defmodule NifIo do
  # TODO: Make this function more File.open
  def open(path) when is_binary(path) do
    options = %NifIo.Native.FileOpenOptions{path: path}

    case NifIo.Native.open(options) do
      {:error, err} -> {:error, err}
      res -> {:ok, NifIo.FileHandle.wrap_resource(res)}
    end
  end

  def read_until(%NifIo.FileHandle{resource: resource}, until_byte)
      when is_number(until_byte) do
    NifIo.Native.read_until(resource, until_byte)
  end
end
