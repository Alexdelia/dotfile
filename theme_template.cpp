#ifndef TMP_HPP
# define TMP_HPP

#include "utils.hpp"
#include <iostream>

int	foo(int ac, char **av, const std::string &tmp)
{
	static	long			sl;
	std::string				s = "Hello Wordl!";
	std::string::iterator	y = s.begin();
	int						x = 1234567890;

	x = x + x;
	x = x - x;
	x = x * x;
	x = x / x;
	x += x;
	x -= x;
	x *= x;
	x /= x;
	x++; ++x;
	x--; --x;
	y++; ++y;
	y--; --y;

	av[0] = NULL;
	av->hey;
	av.hey;
	y->begin();
	y->hey;
	(*(*y)).begin();

	x = &sl;
	x = *sl;

	if ((x == x || x != x) && x == x && sizeof(x))
	while (((((true)))))
		x = false;

	foo(x, NULL, NULL);

	)
	// comment
}
