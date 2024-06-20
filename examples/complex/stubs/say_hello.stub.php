<?php

/** @generate-class-entries */

namespace Complex {
  function say_hello(string $name): string {}

  function throw_exception(): void
  {
  }

  function get_all_ini(): array
  {
  }


  /**
   * @strict-properties
   */
  class Foo {
    private int|\JsonSerializable|\ArrayAccess $foo = 100;
    public function getFoo(): int {}

    public function setFoo(int $foo): void {}
  }
}