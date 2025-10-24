// rustc -C opt-level=3 chris-allen4.rs && ./chris-allen4
// 9800X3D
// v7_maybe_uninit           -> Result: 2472, Avg: 1460 ns
//                              Single run: 1510 ns

// v8_pointer_iteration      -> Result: 2472, Avg: 1452 ns
//                              Single run: 1500 ns

// v9_u8_positions           -> Result: 0, Avg:  223 ns
//                              Single run:  250 ns

// v10_frequency             -> Result: 2472, Avg: 1753 ns
//                              Single run: 1790 ns

// v11_pattern_aware         -> Result: 2472, Avg: 1467 ns
//                              Single run: 1490 ns

use std::mem::MaybeUninit;
use std::time::SystemTime;

const RAW_DATA: &str = "qvllndllhzhfzhhdzhddhjdjggvnvhvccmffwllqgqmmfjfqfhhtrrzczjczzlplddfpptqqfbqffmnmjnnqppfjfccgnnmqqsvvdbbgppjvpjvpjjctjjttwtrrdldlcddrvddqndqnqwqwzwfwwzczggcppgzpzhpzhppprfffbhhwmhhtftstrsrvsrvsrvvshvssnwwpllhfhnnfflcltlblzlqlvqlvlcldcccpptggtdgdjdbbrggmbmnncscbssqrrjddvcvgvfflpppgpvphvphhpcpzpzvvctvctvthtwtfwwbrrhhlplmlwwlqlnlhhtmhmmqlqplllrvrgvrvrffzfgfjfjtjmjvmjmwmvvjffmpfphfhvfvmfvmmhpphhltthgttgccqggpzpfpqpcpvcpvcvvqtqvqbbrlrtllmrmllhmhvmhhvzhvzvrrrzjzbbtvvbgvbbfnnqndqnnpnbnbnlnggwqggmgmqmgmbbmccgqcqbccpvcvnnhvvrvlrrcwrcwrcwrwbrwwzbwbdbfddpttntzzjszsnznbndnzngzgccjrcjchcffmlmqqlrqqzsqzzsbsnsttzpztpzpggzrrttbqqplpqlqjjqcqvccdzdccthccvfcvvqvhqhfhhzwzpzwppgpttntssflfjjrwrqrjrppptlltptpvttpfpwpswpppzzsrzssqllbnlljpllrjllsrlrhrdrmdrmrrpsrprnrffgrffdqdhdqhhrhggwqqlddsbsqbqtqdtdhdvhhbdhdzhdhhtrrppzddgfgzgpzpvpfpnpptggltggbnbppqffzfrzzzsbsrrdgrddwsdsqddhpdpbpvpfvppfsfgfngffzmzbzlblclsccvqvqmmjtjqtjjlcjllsddjqddhldlvlrrbgbrgbrrdzzpfpggqnqbqrrqbbgjgppqgpgwgqqndncndnpdnnbvbnvnwnjjgppzlplqqdgqghqgqzggjssqmmwwcfcpptrpprggrppgbplmzwmdtnpqwzcrthqbppwbgcvgqrpfpnbscnhvrllpvpqwnsslcjrqtvdccprvqfrpswtpvzdzlgtmmvppdmhgdbbsmrbqpqspdhpqgfjznqzphrnggcbzhdqrgvzcfzrhtrlssgmjjghqsjtghhnwjffqrrfslfnsvvdvfjqbfpffrrstdhggvbfwtfpfgswqlfdrnjpjmwzptlbmwgghgwqrphcrvfmhrplllgbnjlprllmjwccphsflntgpnbmdbfqcdsbgvrnfznfrlcfvswqfrqvdnbjsflnsmlcrdstzppmcvbgdtcvgztbdzqbwhmwcfvbwjjcdgbnwjwzrrdqhpgscwtnztjsfstzfwftcldjgvdvwbzrlbdslwttbqpnlwbjcjwqgtrgcglsgtdqbqbnqznptzzbwffwlwzvvtdpcjbvhnswzptclpbndcdvsfmcrmwwgzdfsszqjjdztmtsqgfqzjpctfdpwnzbpnzzwngqnghntblndfrnjzdrmgbqmzbdqfzctrgshwqgfgqssqjltrqlzjswjhmpgwwjdwcjpnsvgrvbfpmlmmwzmbdjwsrjthppfrccjgnmwlvqlprgslbwtbbzlqbznczmsmhsfdcqnwblprcpbzzwfllbnldvpjcwsdhglrzjsptmsjdjqzsmgvhjfjrrtvvbjlmzjsntnrggwbpjlrjggfgqzvswtggthzfmfjnmrzrttbzqpwpsnmdtnbfblpfgslgcmjlbdpshnnrbhvwsbrnvdmjqhvhdjhbfzjmqrmqmdthhzvnrmqcnbtwcdjdqfvdgvmfbhrfqnmdncrddggtcppjlznbsnntppjtnsqsrjwvfrzpnzqcrzhhdflfmmtmwcvtpzbqhdwsczffcqhtdbdjblmgnrmhlqcsvcpgghhvwqhdtzpzlpfllchzltqgcwgfqnbzhgzmdwqdlwnvhqmpqjqnjbhjctslghdqvctdmjfwdfpdjnhdndzwsfjzlmsbmfmzvnvpqgqhtngvgqmlrrzsfmwlcwsscvghjvrzjjqbnplnjzqswpblwzwczhwbhhnjmctnmwlbqqfmnlwdcrptlmfjpjrnpcvmhffjhwhmntdzpdjzwzhrrsdvmjlwdtcpvjfmfzfsrgjghhlvmjjjczgmhvrfpgqbnhldwbrjgzmnszzbssfzcggrwmdfvddwsdmnwtwfwlfnwlvzlctfblbtrjvcwjjdljplcrjhwqslppwwtvfqwsjlfmdznmcdzdmgvmmsrfcclcvhtrhlsjzrbjwrjlfnvqhqvmpzmdttnbhfcvnqlrqbcsvtvwfccjstjpmhqgwlnrzjjmfdszflmglrdbpqhqhqsdfzrcljbdvvnlcqfllmnqcjfzjppdsjwshfschzqbnwfqnpwhqnmwsjbtcgvrljsrtzvcvghcjjlqsngglcggqpntrrhbjpbfhmvpltmnfmfdtwnczwfbvjcqnhvppjftwvwsrlhvvcjtsfptpqgrmrqwwddnqmnmfgrlnphbpqhhhvglqgtwvnwvnbssftmwttmfrffwtzhrpqspclvgchwqwcsgwqwwvpgcwngrcfmhbhflwfbfchlphdzdcrflfmfclsngtlwrqcrsgrdzcpdsvvcdbhgtljmbntbbcqgjqfsbfwzlfsnljpjdcnmjlqrwpmlvwgdlrrdgfhdqhzgltmclzgzzhmrbggsmgtpqdrgmjtlzwstrwbpvhppvsmdqvvwwglzjgdswjszqmrdbmshbhhcstpcsjdbvgjnvcmvhbtclrlmlgnvppgvncsrfchdbqjrclwwlnchmcgvshfsbsvvcvjrsgjlnsfqtqmgntffwnqjtldcqbcqhsgztllstswwqnfrswpchqhnfzzzszqjztzfrgrbjdbjlpvqfqrlrmmpbfbbcclrgmnlzwqrjhqrstswjpgsrtnlwsbqthzpvdzllzqmdmbvvtcztftvlwphhjzbfnrvccfmhmvmzlbrzlnppfzcsffjvjmbgpvlwgwszpztjpsrbnftqtdrbnljtbrjzzbwlsvtwtlwptdtnmtncvcblcmdngjzmctlqtzchncccnwjzrrmmmnllbhrnhwtqjsnvcslrqjfbfndqvdlrjshdzmlprtzbtnhthdqhplwzdbnjmgzlzrbzrvrqnflwfmsmbssqnbcddnvdpltpmplpdzvtjrslcdcnrdplwtjtvctwfzhlvwwqqtbqcjjwhhnpmvgzhqmqfgthwbphrmrtdghchsmwghdqjgjgmpddbrtngtvhqgjfrplrdgpbnhqvswrmqhcmsqvsqmqsgwjndwjrbrhvrctmmrmfwpsgfgdlrzpslpflgvwrgcthgcrnhgrzsmqdgdssjgspfhmqfmjfpmwqhnfjdvqzhpndvnbmqglbrjmdrwgmgctrgzpsdvfbmcstcslblmvnprphntgslmlrqwthrndrhtbccgzzfsglhgqztcsnqjwfzbzlvrpbvswbhrwdsrhrrpnrmsbvbvjccbdsdcfrrzpgwjtnnnvjwlcppwzdqsbdzpfjplrlfgvjpsmbzwpwlghnvqgddfjvrsztrpzlfgmqqzrfcgglghndbhgbmldglclhldljjdslvhzshshtqwhqnbzhvqrcmwdmcmhjcrmdmhrwnwcbhvbbrwrbtfdnztwnbpdfjfhgrmcpngftsvbsmsptnwcvvllnmbnsntbzmwnhfdptbtzswtjzdqwjdhprnjwvhzpscjvlsgrhdrmmrmhzhwwtslzdjqmzfncnmgplhnmwrvqhslvchtjcmpzpjpnpfbjptvvwcsmhgdjtsqrjlfpnfdncpqqmpgpvtlvwljlsqbnhtsqgfwlsmdjpgtvgjvjcrnnzmbllqzlrfdnlffgmtphhhgbcjgdlpzqpwmjwtcmdrsmtnmddftwczbsddtppsptbwfvpnfnsqmsgcfqfmnzffzqgcdvwzrgdwhmnzmrlhcdpdsltnsmjzdqwmmpwvjqbbwsrfgzh";
const DATA: &[u8] = RAW_DATA.as_bytes();

// Version 7: MaybeUninit to avoid initialization
#[inline(always)]
pub fn v7_maybe_uninit(input: &[u8]) -> u32 {
    if input.len() < 14 {
        return 0;
    }

    let mut last_pos: MaybeUninit<[i16; 256]> = MaybeUninit::uninit();

    unsafe {
        let last_pos_ptr = last_pos.as_mut_ptr() as *mut i16;

        // Fast memset to -1
        for i in 0..256 {
            *last_pos_ptr.add(i) = -1;
        }

        let mut window_start: i16 = 0;

        for i in 0..input.len() {
            let c = *input.get_unchecked(i) as usize;
            let i16_pos = i as i16;

            let last = *last_pos_ptr.add(c);
            if last >= window_start {
                window_start = last + 1;
            }

            *last_pos_ptr.add(c) = i16_pos;

            if i16_pos - window_start >= 13 {
                return (i + 1) as u32;
            }
        }
    }

    0
}

// Version 8: Raw pointer iteration
#[inline(always)]
pub fn v8_pointer_iteration(input: &[u8]) -> u32 {
    if input.len() < 14 {
        return 0;
    }

    let mut last_pos: [i16; 256] = [-1; 256];
    let mut window_start: i16 = 0;

    unsafe {
        let mut ptr = input.as_ptr();
        let end = ptr.add(input.len());
        let mut i = 0i16;

        while ptr < end {
            let c = *ptr as usize;

            let last = last_pos[c];
            if last >= window_start {
                window_start = last + 1;
            }

            last_pos[c] = i;

            if i - window_start >= 13 {
                return (i + 1) as u32;
            }

            ptr = ptr.add(1);
            i += 1;
        }
    }

    0
}

// Version 9: Using u8 with wrapping for positions (smaller memory footprint)
#[inline(always)]
pub fn v9_u8_positions(input: &[u8]) -> u32 {
    if input.len() < 14 {
        return 0;
    }

    let mut last_pos: [u8; 256] = [255; 256];
    let mut window_start: u8 = 0;

    for i in 0..input.len().min(256) {
        let c = input[i] as usize;
        let i_u8 = i as u8;

        let last = last_pos[c];
        if last != 255 && last.wrapping_sub(window_start) < 128 {
            window_start = last.wrapping_add(1);
        }

        last_pos[c] = i_u8;

        let window_size = i_u8.wrapping_sub(window_start).wrapping_add(1);
        if window_size >= 14 {
            return (i + 1) as u32;
        }
    }

    0
}

// Version 10: Frequency counting approach
#[inline(always)]
pub fn v10_frequency(input: &[u8]) -> u32 {
    if input.len() < 14 {
        return 0;
    }

    let mut freq: [u8; 256] = [0; 256];
    let mut unique = 0u8;

    unsafe {
        // Fill initial window
        for i in 0..14 {
            let c = *input.get_unchecked(i) as usize;
            if freq[c] == 0 {
                unique += 1;
            }
            freq[c] += 1;
        }

        if unique == 14 {
            return 14;
        }

        // Slide window
        for i in 14..input.len() {
            // Remove leftmost
            let old_c = *input.get_unchecked(i - 14) as usize;
            freq[old_c] -= 1;
            if freq[old_c] == 0 {
                unique -= 1;
            }

            // Add rightmost
            let new_c = *input.get_unchecked(i) as usize;
            if freq[new_c] == 0 {
                unique += 1;
            }
            freq[new_c] += 1;

            if unique == 14 {
                return (i + 1) as u32;
            }
        }
    }

    0
}

// Version 11: Hybrid with early exit on common patterns
#[inline(always)]
pub fn v11_pattern_aware(input: &[u8]) -> u32 {
    if input.len() < 14 {
        return 0;
    }

    let mut last_pos: [i16; 256] = [-1; 256];
    let mut window_start: i16 = 0;

    unsafe {
        for i in 0..input.len() {
            let c = *input.get_unchecked(i) as usize;
            let i16_pos = i as i16;

            let last = *last_pos.get_unchecked(c);

            // Check if duplicate in window
            let is_duplicate = last >= window_start;
            window_start = if is_duplicate { last + 1 } else { window_start };

            *last_pos.get_unchecked_mut(c) = i16_pos;

            // Use subtraction for faster comparison
            let window_size = i16_pos - window_start;
            if window_size >= 13 {
                return (i + 1) as u32;
            }
        }
    }

    0
}

fn benchmark<F: Fn(&[u8]) -> u32>(name: &str, f: F) {
    // Warmup
    for _ in 0..1000 {
        let _ = f(DATA);
    }

    // Benchmark
    let iterations = 1_000_000;
    let before = SystemTime::now();
    let mut result = 0;
    for _ in 0..iterations {
        result = f(DATA);
    }
    let after = SystemTime::now();

    let total_time = after.duration_since(before).unwrap();
    let avg_nanos = total_time.as_nanos() / iterations;

    println!("{:25} -> Result: {}, Avg: {:4} ns", name, result, avg_nanos);

    // Single run
    let before = SystemTime::now();
    let _result = f(DATA);
    let after = SystemTime::now();
    let single = after.duration_since(before).unwrap().as_nanos();
    println!("{:25}    Single run: {:4} ns\n", "", single);
}

fn main() {
    benchmark("v7_maybe_uninit", v7_maybe_uninit);
    benchmark("v8_pointer_iteration", v8_pointer_iteration);
    benchmark("v9_u8_positions", v9_u8_positions);
    benchmark("v10_frequency", v10_frequency);
    benchmark("v11_pattern_aware", v11_pattern_aware);
}
