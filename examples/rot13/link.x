MEMORY
{
	APPLICATION : ORIGIN = 0x8000, LENGTH = 640k
}

ENTRY(main);

STACK_SIZE = 16384;

SECTIONS
{
	.text :
	{
		KEEP(*(.text.entry));

		*(.text*);
	} > APPLICATION

	.data :
	{
		*(.rodata*)
		*(.data*)
	} > APPLICATION

	.ARM.exidx :
	{
		*(.ARM.exidx)
	} > APPLICATION

	.bss (NOLOAD) :
	{
		*(.bss*)
	} > APPLICATION

	.stack (NOLOAD) :
	{
		. = ALIGN(4);
		. = . + STACK_SIZE;
		. = ALIGN(4);
		_stack_end = .;
	} > APPLICATION
}
