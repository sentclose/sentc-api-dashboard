export function p(item: string)
{
	return (process.env.NODE_ENV === "production" ? "/dashboard/" : "/") + item;
}