# RULES

if '(': create empty new exp below in tree
if ')': go up to last empty exp in tree
if op and current exp has no op: put op in current exp
if op and current exp has op:
		{
if less important (BIMDAS): put above current in tree
if more: move current exp left and put below
		}