use dialoguer::{Confirm, Input};
use printpdf::{BuiltinFont, Mm, PdfDocument, Pt};
use std::fs::File;
use std::io::BufWriter;
use textwrap::wrap;

fn main() {
    let cl = String::from("MD Rabby Khan\nwapda Kolabagan, Hetemkha, Boalia\nRajshahi, 6000\n[Date]\n\n[Hiring Manager’s Name]\n[COMPANY NAME]\n[Company Address]\n[City, State, ZIP Code]\n\nSubject: Application for [ROLE NAME] at [COMPANY NAME]\n\nDear Hiring Manager at [COMPANY NAME],\n\nI am excited to apply for the [ROLE NAME] position at [COMPANY NAME]. As a passionate Full Stack Web Developer with hands-on experience in modern web technologies, I am eager to contribute my skills to your mission of revolutionizing the [INDUSTRY TYPE].\n\nWith a strong foundation in JavaScript, React.js, Next.js, Node.js, and Express.js, I have developed full-stack applications that emphasize efficiency, security, and scalability. My experience with MongoDB, PostgreSQL, and Prisma ORM has further enhanced my ability to build robust backend solutions.\n\nBeyond technical expertise, I thrive in collaborative environments where I can learn from experienced mentors and contribute innovative solutions. My commitment to continuous learning is evident through my recent certification in Next.js & Prisma, and I am particularly excited about the opportunity to work with cutting-edge technologies at [COMPANY NAME].\n\nI would welcome the opportunity to discuss how my skills and enthusiasm align with [COMPANY NAME]'s vision. Thank you for your time and consideration. I look forward to the possibility of contributing to your team.\n\nYours Sincerely,\n\nMD Rabby Khan\nPhone: +8801796382985\nEmail: rabbykhan082020@gmail.com\nLinkedIn: www.linkedin.com/in/md-rabby-khan-89b7gyhy787g\nPortfolio: https://portfolio-eosin-xi-96.vercel.app/");

    // Get user inputs
    let date: String = Input::new().with_prompt("Date").interact_text().unwrap();
    let company: String = Input::new()
        .with_prompt("Company Name")
        .interact_text()
        .unwrap();
    let role: String = Input::new().with_prompt("Role").interact_text().unwrap();
    let industry_type: String = Input::new()
        .with_prompt("Industry Type")
        .interact_text()
        .unwrap();

    let add_hiring_manager_details = Confirm::new()
        .with_prompt("Do you want to include the Hiring Manager's details? (y/n)")
        .interact()
        .unwrap();

    let (manager_name, company_address, company_point) = if add_hiring_manager_details {
        let manager_name: String = Input::new()
            .with_prompt("Hiring Manager's Name")
            .interact_text()
            .unwrap();
        let company_address: String = Input::new()
            .with_prompt("Company Address (Area, Thana, City)")
            .interact_text()
            .unwrap();
        let company_point: String = Input::new()
            .with_prompt("Company Address (House No, Block, ZIP)")
            .interact_text()
            .unwrap();
        (manager_name, company_address, company_point)
    } else {
        ("Hiring Manager".to_string(), "".to_string(), "".to_string())
    };

    // Replace placeholders with actual user inputs
    let update_cl = cl
        .replace("[Date]", &date)
        .replace("[COMPANY NAME]", &company)
        .replace("[ROLE NAME]", &role)
        .replace("[INDUSTRY TYPE]", &industry_type)
        .replace("[Hiring Manager’s Name]", &manager_name)
        .replace("[Company Address]", &company_address)
        .replace("[City, State, ZIP Code]", &company_point);

    // Create the PDF document
    let (mut doc, page1, layer1) =
        PdfDocument::new("Cover Letter", Mm(210.0), Mm(297.0), "Layer 1");

    // Get the actual PDF layer
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Define font
    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();

    // Wrap text
    let max_width = 80; // Max characters per line
    let wrapped_lines = wrap(&update_cl, max_width);

    // Add wrapped lines to the layer
    let mut y_offset = 270.0; // Start near the top of the page
    for line in wrapped_lines {
        current_layer.use_text(line, 12.0, Mm(20.0), Mm(y_offset), &font);
        y_offset -= 5.0; // Move to the next line down
    }

    // Open file and wrap it in a BufWriter
    let file = File::create("cover_letter.pdf").unwrap();
    let mut writer = BufWriter::new(file);

    // Save the document
    doc.save(&mut writer).unwrap();

    println!("PDF file 'cover_letter.pdf' created successfully!");
}
