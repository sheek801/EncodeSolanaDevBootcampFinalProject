'use client';

import { AppHero } from '../ui/ui-layout';
import { QueryClient, useQuery } from '@tanstack/react-query';

const links: { label: string; href: string }[] = [
  { label: 'Solana Docs', href: 'https://docs.solana.com/' },
  { label: 'Solana Faucet', href: 'https://faucet.solana.com/' },
  { label: 'Solana Cookbook', href: 'https://solanacookbook.com/' },
  { label: 'Solana Stack Overflow', href: 'https://solana.stackexchange.com/' },
  {
    label: 'Solana Developers GitHub',
    href: 'https://github.com/solana-developers/',
  },
];

export default function DashboardFeature() {
  const { isLoading, error, data } = useQuery({
    queryKey: ['action'],
    queryFn: () =>
      fetch('http://localhost:5000/price/action').then(
        (res) => res.json()
      ),
  });

  const solanaPrice = data?.solanaPrice;
  const premium120 = data?.premium120;
  const premium200 = data?.premium200;
  const finalPremium = data?.finalPremium;

  return (
    <div>
      <AppHero title="gm" subtitle="Say hi to your new Solana dApp." />
      <div className="max-w-xl mx-auto py-6 sm:px-6 lg:px-8 text-center">
        <div className="space-y-2">
          {!isLoading && <div>{Math.round(finalPremium * 1000) / 1000} %</div>}
          <p>Here are some helpful links to get you started.</p>
          {links.map((link, index) => (
            <div key={index}>
              <a
                href={link.href}
                className="link"
                target="_blank"
                rel="noopener noreferrer"
              >
                {link.label}
              </a>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
