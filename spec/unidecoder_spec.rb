require 'unidecode_rust/unidecode_rust'
describe 'Unidecoder' do
  it 'converts Greek letters to Latin letters' do
    expect(Unidecoder.unidecode 'Καλημέρα').to eq 'Kalemera'
  end
end
